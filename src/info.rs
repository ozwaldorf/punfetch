use byte_unit::Byte;
use colored::*;
use humantime::format_duration;
use std::fmt::Display;
use std::string::ToString;
use std::time::Duration;
use sysinfo::{
    CpuExt, Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, UserExt,
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

pub fn sys_info() -> Vec<String> {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_cpu(Default::default()));
    sys.refresh_all();
    let cpus = sys.cpus();
    fmt(vec![
        (
            "OS".green().bold(),
            sys.name().unwrap_or_else(|| sys.distribution_id()),
        ),
        (
            "Kernel".green().bold(),
            sys.kernel_version().unwrap_or_else(|| "Unknown".into()),
        ),
        ("CPU".green().bold(), cpus[0].brand().to_string()),
        ("Core Count".green().bold(), cpus.len().to_string()),
        (
            "RAM".green().bold(),
            format!(
                "{} » {}",
                Byte::from_bytes(sys.used_memory() as u128).get_appropriate_unit(true),
                Byte::from_bytes(sys.total_memory() as u128).get_appropriate_unit(true)
            ),
        ),
    ])
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
        let info = user_info();
        assert_eq!(info.len(), 2);
        assert!(info[0].contains("up"));
    }

    #[test]
    fn test_sys_info() {
        let info = sys_info();
        assert_eq!(info.len(), 5);
        assert!(info[0].contains("OS"));
    }

    #[test]
    fn test_colors() {
        let info = colors();
        assert_eq!(info.len(), 1);
        assert!(info[0].contains("   "));
    }
}
