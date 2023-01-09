use byte_unit::Byte;
use colored::{Color, Colorize};
use humantime::format_duration;
use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::process::id;
use std::string::ToString;
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, Pid, PidExt, ProcessExt, System, SystemExt, UserExt};

pub fn user_info(sys: &System) -> Vec<String> {
    let host = sys.host_name().unwrap_or("localhost".to_string());
    let mut len = host.len();
    let mut line = String::new();
    if let Some(user) = sys.process(Pid::from_u32(id())).and_then(|p| {
        p.user_id()
            .and_then(|u| sys.get_user_by_id(u).map(|u| u.name()))
    }) {
        len += user.len() + 3;
        line.push_str(&format!("{} ~ ", user.bold().green()))
    }
    line.push_str(&host.bold().green());

    vec![line, format!("{}", "-".repeat(len))]
}

pub fn os_info(sys: &System) -> Vec<String> {
    let mut buf = vec![];

    let distro = sys.name().unwrap_or_else(|| sys.distribution_id());
    if !distro.is_empty() {
        buf.push(("OS".green().bold(), distro))
    }

    if let Some(kernel) = sys.kernel_version() {
        buf.push(("Kernel".green().bold(), kernel))
    }

    if let Ok(term) = env::var("TERM") {
        buf.push(("Terminal".green().bold(), term))
    }

    fmt(buf)
}

pub fn sys_info(sys: &System) -> Vec<String> {
    let mut buf = vec![];

    let uptime = sys.uptime();
    if uptime != 0 {
        buf.push((
            "Uptime".green().bold(),
            format!("{}", format_duration(Duration::from_secs(uptime))),
        ))
    }

    let cpus = sys.cpus();
    if !cpus.is_empty() {
        buf.push(("CPU".green().bold(), cpus[0].brand().to_string()));

        let load_avg = sys.load_average().fifteen;
        if load_avg != 0.0 {
            buf.push((
                "Load".green().bold(),
                format!("~{}% ({} cores)", sys.load_average().fifteen, cpus.len()),
            ))
        }
    }

    let used_mem = sys.used_memory() as u128;
    let total_mem = sys.total_memory() as u128;
    if used_mem + total_mem != 0 {
        buf.push((
            "RAM".green().bold(),
            format!(
                "{} / {} ({:.01} %)",
                Byte::from_bytes(used_mem).get_appropriate_unit(true),
                Byte::from_bytes(total_mem).get_appropriate_unit(true),
                used_mem as f64 / total_mem as f64 * 100.0
            ),
        ))
    }

    fmt(buf)
}

pub fn disk_info(sys: &System, size: usize) -> Vec<String> {
    let color_palette = vec![
        Color::BrightMagenta,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightRed,
        Color::White,
        Color::BrightBlack,
    ];

    let mut seen = HashSet::new();
    let mut total = 0.0;
    let mut disk_info: Vec<(String, f64, f64, Color)> = vec![];
    let disks = sys.disks();
    if disks.is_empty() {
        return vec![];
    } else {
        for disk in disks.iter() {
            if seen.insert(disk.name().to_str().unwrap()) {
                let disk_total = disk.total_space() as f64;
                total += disk_total;
                let used = disk_total - disk.available_space() as f64;
                let color = color_palette[seen.len() - 1 % color_palette.len()];
                disk_info.push((
                    disk.mount_point().to_str().unwrap().to_string(),
                    used / disk_total,
                    disk_total,
                    color,
                ));
            }
        }
    }

    // build bar and lines 2 at a time
    let mut bar = format!("{}: ", "Disks".green().bold());
    let mut lines = Vec::new();
    let mut remainder = size;
    for info in disk_info.chunks(2) {
        // initial padding for info to align with "Disks: "
        let mut line = "       ".to_string();
        for (mount, percent, disk_total, color) in info {
            let max_width = disk_total / total * size as f64;
            let mut width = (percent * max_width) as usize;
            if width == 0 && *percent > 0.01 {
                // make sure we can see something for disks > 1% usage
                width += 1;
            }
            remainder -= width;
            // push section to bar
            bar.push_str(&format!("{:>width$}", "".on_color(*color),));
            // push section to current line
            line.push_str(&format!(
                "{} {mount} ({:.1} %) ",
                "●".color(*color),
                percent * 100.0
            ));
        }
        // push both info to lines
        lines.push(line);
    }
    // fill the remainder of the bar
    bar.push_str(&format!("{:>remainder$}", "".on_black(),));

    let mut buf = vec![bar];
    buf.extend(lines);
    buf
}

pub fn colors() -> Vec<String> {
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

fn fmt<K: Display, V: Display>(values: Vec<(K, V)>) -> Vec<String> {
    let mut buf = Vec::new();
    for (key, value) in values {
        buf.push(format!("{key}: {value}"));
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info() {
        let sys = System::new_all();
        for line in user_info(&sys) {
            println!("{line}");
        }
    }

    #[test]
    fn test_os_info() {
        let sys = System::new_all();
        for line in os_info(&sys) {
            println!("{line}");
        }
    }

    #[test]
    fn test_disks() {
        let sys = System::new_all();
        for line in disk_info(&sys, 32) {
            println!("{line}");
        }
    }

    #[test]
    fn test_sys_info() {
        let sys = System::new_all();
        for line in sys_info(&sys) {
            println!("{line}");
        }
    }

    #[test]
    fn test_colors() {
        for line in colors() {
            println!("{line}");
        }
    }
}
