use byte_unit::Byte;
use colored::*;
use humantime::format_duration;
use std::collections::HashSet;
use std::fmt::Display;
use std::string::ToString;
use std::time::Duration;
use sysinfo::{
    CpuExt, DiskExt, Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt,
    UserExt,
};

pub fn user_info() -> Vec<String> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_processes(ProcessRefreshKind::everything())
            .with_users_list(),
    );
    sys.refresh_all();
    let host = sys.host_name().unwrap_or_default();
    let uid = sys
        .process(Pid::from_u32(std::process::id()))
        .expect("Failed to get current process")
        .user_id()
        .expect("Failed to get current user id");
    let user = sys
        .get_user_by_id(uid)
        .expect("Failed to get current user")
        .name();
    let uptime = format_duration(Duration::from_secs(sys.uptime())).to_string();
    let len = user.len() + host.len() + uptime.len() + 9;
    vec![
        format!(
            "{} ~ {} ({})",
            user.bold().green(),
            host.bold().green(),
            format!("up {uptime}").green()
        ),
        format!("{}", "-".repeat(len)),
    ]
}

pub fn os_info(sys: &System) -> Vec<String> {
    fmt(vec![
        (
            "OS".green().bold(),
            sys.name().unwrap_or_else(|| sys.distribution_id()),
        ),
        (
            "Kernel".green().bold(),
            sys.kernel_version().unwrap_or_else(|| "Unknown".into()),
        ),
    ])
}

pub fn sys_info(sys: &System) -> Vec<String> {
    let cpus = sys.cpus();
    let used_mem = sys.used_memory() as u128;
    let total_mem = sys.total_memory() as u128;
    fmt(vec![
        ("CPU".green().bold(), cpus[0].brand().to_string()),
        ("Load".green().bold(), format!("~{} % ({} cores)", sys.load_average().fifteen, cpus.len())),
        (
            "RAM".green().bold(),
            format!(
                "{} / {} ({:.01} %)",
                Byte::from_bytes(used_mem).get_appropriate_unit(true),
                Byte::from_bytes(total_mem).get_appropriate_unit(true),
                used_mem as f64 / total_mem as f64 * 100.0
            ),
        ),
    ])
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
    let mut info: Vec<(String, f64, f64, Color)> = vec![];
    for disk in sys.disks().iter() {
        if seen.insert(disk.name().to_str().unwrap()) {
            let disk_total = disk.total_space() as f64;
            total += disk_total;
            // skip if total space of a partition is less than 1GB
            let used = disk_total - disk.available_space() as f64;
            let color = color_palette[seen.len() - 1 % color_palette.len()];
            info.push((
                disk.mount_point().to_str().unwrap().to_string(),
                used / disk_total,
                disk_total,
                color,
            ));
        }
    }

    let mut bar = format!("{}: ", "Disks".green().bold());
    let mut lines = Vec::new();
    let mut remainder = size;

    for line in info.chunks(2) {
        let mut info_line = "       ".to_string();
        for (mount, percent, disk_total, color) in line {
            let max_width = disk_total / total * size as f64;
            let mut width = (percent * max_width) as usize;
            if width == 0 {
                width += 1;
            }

            remainder -= width;
            bar.push_str(&format!("{:>width$}", "".on_color(*color),));
            info_line.push_str(&format!(
                "{} {mount} ({:.1} %) ",
                "●".color(*color),
                percent * 100.0
            ));
        }
        lines.push(info_line);
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
        buf.push(format!("{}: {}", key, value));
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info() {
        for line in user_info() {
            println!("{}", line);
        }
    }

    #[test]
    fn test_os_info() {
        let mut sys = System::new_all();
        sys.refresh_all();
        for line in os_info(&sys) {
            println!("{}", line);
        }
    }

    #[test]
    fn test_disks() {
        let mut sys = System::new_all();
        sys.refresh_all();
        for line in disk_info(&sys, 32) {
            println!("{}", line);
        }
    }

    #[test]
    fn test_sys_info() {
        let mut sys = System::new_all();
        sys.refresh_all();
        for line in sys_info(&sys) {
            println!("{}", line);
        }
    }

    #[test]
    fn test_colors() {
        for line in colors() {
            println!("{}", line);
        }
    }
}
