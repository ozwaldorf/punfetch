use std::{
    collections::HashSet, default::Default, env, process::id, string::ToString, time::Duration,
};

use byte_unit::Byte;
use humantime::format_duration;
use owo_colors::{colored::Color, DynColors, OwoColorize};

#[cfg(feature = "sysinfo")]
use sysinfo::{
    CpuExt, CpuRefreshKind, Disk, DiskExt, Pid, PidExt, ProcessExt, ProcessRefreshKind,
    RefreshKind, System, SystemExt, UserExt,
};

use super::*;

pub const DEFAULT_BAR_WIDTH: usize = 30;
pub const DEFAULT_COLORS: [DynColors; 7] = [
    DynColors::Ansi(Color::Red),
    DynColors::Ansi(Color::Green),
    DynColors::Ansi(Color::Yellow),
    DynColors::Ansi(Color::Blue),
    DynColors::Ansi(Color::Magenta),
    DynColors::Ansi(Color::Cyan),
    DynColors::Ansi(Color::White),
];

/// Generic color bar
pub struct ColorBar(pub Vec<DynColors>);

impl Default for ColorBar {
    fn default() -> Self {
        let mut colors = vec![DynColors::Ansi(Color::Black)];
        colors.extend(DEFAULT_COLORS);
        Self(colors)
    }
}

impl Render for ColorBar {
    fn render(&self, _: DynColors) -> Vec<String> {
        let mut buf = String::new();
        for color in &self.0 {
            buf.push_str(&format!("{}", "   ".on_color(*color)));
        }
        vec![String::new(), buf]
    }
}

/// Generic percentage bar. Requires a total, a vector of items, and some colors to cycle.
pub struct PercentBar {
    pub title: String,
    pub total: f64,
    pub items: Vec<(String, f64, f64)>,
    pub colors: Vec<DynColors>,
    pub width: usize,
}

impl Render for PercentBar {
    fn render(&self, color: DynColors) -> Vec<String> {
        let mut bufs = vec![format!("{}: ", self.title.bold().color(color))];
        let padding = " ".repeat(self.title.len() + 1);
        let mut remainder = self.width;
        for (i, (name, used, percent)) in self.items.iter().enumerate() {
            let color = self.colors[i % self.colors.len()];
            // calculate width of the section of bar
            let total_percent = used / self.total;
            let mut width = (total_percent * self.width as f64) as usize;
            if width == 0 && total_percent > 0.01 {
                // make sure we can see something for disks > 1% usage
                width += 1;
            }
            remainder -= width;

            // push section to bar
            bufs[0].push_str(&format!("{:>width$}", "".on_color(color)));

            // push name to line
            let name = format!(" {} {name} ({:.1} %) ", "●".color(color), percent * 100.0);
            if let Some(buf) = bufs.get_mut(i / 2 + 1) {
                buf.push_str(&name)
            } else {
                bufs.push(format!("{padding}{name}"))
            }
        }
        // push remainder to bar
        bufs[0].push_str(&format!("{:>remainder$}", "".on_color(Color::Black)));

        bufs
    }
}

#[cfg(feature = "sysinfo")]
/// Disk usage bar with mountpoints and percents
impl From<&[Disk]> for PercentBar {
    fn from(disks: &[Disk]) -> Self {
        let mut seen = HashSet::new();
        let mut total = 0.0;
        let mut items = vec![];

        for disk in disks {
            if seen.insert(disk.name()) {
                let disk_total = disk.total_space() as f64;
                let used = disk_total - disk.available_space() as f64;
                total += disk_total;
                items.push((
                    disk.mount_point().to_string_lossy().to_string(),
                    used,
                    used / disk_total,
                ));
            }
        }

        PercentBar {
            title: "Disks".to_string(),
            total,
            items,
            colors: DEFAULT_COLORS.to_vec(),
            width: DEFAULT_BAR_WIDTH,
        }
    }
}

#[cfg(feature = "sysinfo")]
/// User and host, with a horizontal line underneath
pub struct UserInfo {
    pub user: String,
    pub host: String,
}

impl UserInfo {
    pub fn new(sys: &System) -> Self {
        let user = sys
            .process(Pid::from_u32(id()))
            .and_then(|p| {
                p.user_id()
                    .and_then(|u| sys.get_user_by_id(u).map(|u| u.name().to_string()))
            })
            .unwrap_or_else(|| env::var("USER").unwrap_or_else(|_| "unknown".into()));
        let host = sys.host_name().unwrap_or_else(|| "localhost".to_string());

        Self { user, host }
    }
}

#[cfg(feature = "sysinfo")]
impl Render for UserInfo {
    fn render(&self, color: DynColors) -> Vec<String> {
        let len = self.user.len() + self.host.len() + 3;
        vec![
            format!(
                "{} ~ {}",
                self.user.bold().color(color),
                self.host.bold().color(color)
            ),
            format!("{}", "-".repeat(len).color(Color::Default)),
        ]
    }
}

#[cfg(feature = "sysinfo")]
/// Distro, kernel, and shell
#[derive(Render)]
pub struct HostInfo {
    pub distro: String,
    pub kernel: Option<String>,
    pub terminal: Option<String>,
}

impl HostInfo {
    pub fn new(sys: &System) -> Self {
        Self {
            distro: sys.name().unwrap_or_else(|| sys.distribution_id()),
            kernel: sys.kernel_version(),
            terminal: env::var("TERM").ok(),
        }
    }
}

#[cfg(feature = "sysinfo")]
/// Last boot, cpu brand, avg load, memory usage
#[derive(Render)]
pub struct SystemInfo {
    pub last_boot: String,
    pub cpu: Option<String>,
    pub avg_load: String,
    pub memory: Option<String>,
}

#[cfg(feature = "sysinfo")]
impl SystemInfo {
    pub fn new(sys: &System) -> Self {
        let uptime = sys.uptime();
        let rounded = uptime - uptime % 60;
        let last_boot = format!(
            "{} ago",
            if rounded > 60 {
                format_duration(Duration::from_secs(rounded)).to_string()
            } else {
                "less than one minute".to_string()
            }
        );
        let avg_load = format!("~{:.2} %", sys.load_average().fifteen);

        let cpus = sys.cpus();
        let cpu = if !cpus.is_empty() {
            Some(cpus[0].brand().to_string())
        } else {
            None
        };

        let used_mem = sys.used_memory() as u128;
        let total_mem = sys.total_memory() as u128;
        let percent = (used_mem as f64 / total_mem as f64) * 100.0;
        let memory = if percent != 0.0 {
            Some(format!(
                "{} / {} ({percent:.01} %)",
                Byte::from_bytes(used_mem).get_appropriate_unit(true),
                Byte::from_bytes(total_mem).get_appropriate_unit(true),
            ))
        } else {
            None
        };

        Self {
            last_boot,
            cpu,
            avg_load,
            memory,
        }
    }
}

#[cfg(feature = "sysinfo")]
/// Returns a [`System::new_with_specifics`] configured for the provided structs
pub fn sys() -> System {
    System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new())
            .with_users_list()
            .with_processes(ProcessRefreshKind::new().with_user())
            .with_disks_list()
            .with_memory(),
    )
}

#[cfg(test)]
mod tests {
    use owo_colors::{AnsiColors, AnsiColors::Default};

    use super::*;

    static COLORS: [DynColors; 3] = [
        DynColors::Ansi(AnsiColors::Green),
        DynColors::Ansi(AnsiColors::Yellow),
        DynColors::Ansi(AnsiColors::Red),
    ];

    fn _render<R: Render>(i: R) {
        let lines = i.render(DynColors::Ansi(Default));
        assert!(!lines.is_empty());
        for line in lines {
            println!("{line}");
        }
    }

    #[cfg(feature = "sysinfo")]
    #[test]
    fn host_info() {
        let sys = sys();
        _render(HostInfo::new(&sys));
    }

    #[cfg(feature = "sysinfo")]
    #[test]
    fn user_info() {
        let sys = sys();
        _render(UserInfo::new(&sys));
    }

    #[cfg(feature = "sysinfo")]
    #[test]
    fn sys_info() {
        let sys = sys();
        _render(SystemInfo::new(&sys));
    }

    #[cfg(feature = "sysinfo")]
    #[test]
    fn disk_info() {
        let sys = sys();
        _render(PercentBar::from(sys.disks()));
    }

    #[test]
    fn generic_percent_bar() {
        _render(PercentBar {
            title: "Generic".to_string(),
            total: 100.0,
            items: vec![
                ("foo".to_string(), 50.0, 1.0),
                ("bar".to_string(), 25.0, 0.25),
                ("baz".to_string(), 13.0, 0.13),
            ],
            colors: COLORS.to_vec(),
            width: DEFAULT_BAR_WIDTH,
        });

        _render(PercentBar {
            title: "Full   ".to_string(),
            total: 300.0,
            items: vec![
                ("large".to_string(), 100.0, 1.0),
                ("medium".to_string(), 100.0, 1.0),
                ("small".to_string(), 100.0, 1.0),
            ],
            colors: COLORS.to_vec(),
            width: DEFAULT_BAR_WIDTH,
        });
    }

    #[test]
    fn generic_colors() {
        _render(ColorBar::default());

        let mut colors = DEFAULT_COLORS.to_vec();
        colors.extend(DEFAULT_COLORS);
        _render(ColorBar(colors));
    }
}
