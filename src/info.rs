use std::{collections::HashSet, env, process::id, string::ToString, time::Duration};

use byte_unit::Byte;
use humantime::format_duration;
use owo_colors::{colored::Color, DynColors, OwoColorize};
use sysinfo::{
    CpuExt, CpuRefreshKind, Disk, DiskExt, Pid, PidExt, ProcessExt, ProcessRefreshKind,
    RefreshKind, System, SystemExt, UserExt,
};

use super::*;

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

/// Last boot, cpu brand, avg load, memory usage
#[derive(Render)]
pub struct SystemInfo {
    pub last_boot: String,
    pub cpu: Option<String>,
    pub avg_load: String,
    pub memory: Option<String>,
}

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

/// Disk usage bar with mountpoints and percents
pub struct DiskInfo {
    pub total: f64,
    pub items: Vec<(String, f64)>,
}

impl From<&[Disk]> for DiskInfo {
    fn from(disks: &[Disk]) -> Self {
        let mut seen = HashSet::new();
        let mut total = 0.0;
        let mut items = vec![];

        for disk in disks {
            if seen.insert(disk.name()) {
                total += disk.total_space() as f64;
                items.push((
                    disk.mount_point().to_string_lossy().to_string(),
                    disk.total_space() as f64 - disk.available_space() as f64,
                ));
            }
        }

        DiskInfo { total, items }
    }
}

impl Render for DiskInfo {
    fn render(&self, color: DynColors) -> Vec<String> {
        let mut bufs = vec![format!("{}: ", "Disks".bold().color(color))];

        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
        ];

        let max_width = 30.0;
        let total = self.total;
        let mut remainder = max_width as usize;
        for (i, (name, used)) in self.items.iter().enumerate() {
            let color = colors[i % colors.len()];
            // calculate width of the section of bar
            let percent = used / total;
            let mut width = (percent * max_width) as usize;
            if width == 0 && percent > 0.01 {
                // make sure we can see something for disks > 1% usage
                width += 1;
            }
            remainder -= width;

            // push section to bar
            bufs[0].push_str(&format!("{:>width$}", "".on_color(color)));

            // push name to line
            let name = format!(
                "       {} {name} ({:.1} %) ",
                "●".color(color),
                percent * 100.0
            );
            if let Some(buf) = bufs.get_mut(i / 2 + 1) {
                buf.push_str(&name)
            } else {
                bufs.push(name);
            }
        }
        // push remainder to bar
        bufs[0].push_str(&format!("{:>remainder$}", "".on_color(Color::Black)));

        bufs
    }
}

/// Color bar!
pub struct ColorInfo;

impl Render for ColorInfo {
    fn render(&self, _: DynColors) -> Vec<String> {
        vec![
            "".to_string(),
            format!(
                "{}{}{}{}{}{}{}{}",
                "   ".on_black(),
                "   ".on_red(),
                "   ".on_green(),
                "   ".on_yellow(),
                "   ".on_blue(),
                "   ".on_magenta(),
                "   ".on_cyan(),
                "   ".on_white()
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use owo_colors::AnsiColors;

    use super::*;

    static COLOR: DynColors = DynColors::Ansi(AnsiColors::Default);

    #[test]
    fn host_info() {
        let sys = sys();
        let info = HostInfo::new(&sys);
        assert!(info.render(COLOR).len() > 0);
    }

    #[test]
    fn user_info() {
        let sys = sys();
        let info = UserInfo::new(&sys);
        assert!(info.render(COLOR).len() > 0);
    }

    #[test]
    fn sys_info() {
        let sys = sys();
        let info = SystemInfo::new(&sys);
        assert!(info.render(COLOR).len() > 0);
    }

    #[test]
    fn disk_info() {
        let sys = sys();
        let info = DiskInfo::from(sys.disks());
        assert!(info.render(COLOR).len() > 0);
    }

    #[test]
    fn colors() {
        let cols = ColorInfo;
        assert!(cols.render(COLOR).len() > 0);
    }
}
