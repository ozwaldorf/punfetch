use std::{collections::HashSet, env, process::id, string::ToString, time::Duration};

use byte_unit::Byte;
use humantime::format_duration;
use sysinfo::{CpuExt, Disk, DiskExt, Pid, PidExt, ProcessExt, System, SystemExt, UserExt};

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

pub struct HostInfo(pub Vec<(&'static str, String)>);

impl HostInfo {
    pub fn new(sys: &System) -> Self {
        let mut buf = vec![];

        if let Some(kernel) = sys.kernel_version() {
            buf.push(("Kernel", kernel))
        }

        if let Ok(term) = env::var("TERM") {
            buf.push(("Terminal", term))
        }

        Self(buf)
    }

    //noinspection DuplicatedCode
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &String)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }
}

#[derive(Clone)]
pub struct DiskInfo {
    pub total: f64,
    pub keys: Vec<String>,
    pub values: Vec<f64>,
}

impl From<&[Disk]> for DiskInfo {
    fn from(disks: &[Disk]) -> Self {
        let mut seen = HashSet::new();
        let mut total = 0.0;
        let mut keys: Vec<String> = vec![];

        let mut values = vec![];

        for disk in disks {
            if seen.insert(disk.name()) {
                total += disk.total_space() as f64;
                keys.push(disk.mount_point().to_string_lossy().to_string());
                values.push(disk.total_space() as f64 - disk.available_space() as f64);
            }
        }

        DiskInfo {
            total,
            keys,
            values,
        }
    }
}

pub struct SystemInfo(pub Vec<(&'static str, String)>);

impl SystemInfo {
    pub fn new(sys: &System) -> Self {
        let mut buf = vec![];

        let uptime = sys.uptime();
        let line = format!("{} ago", format_duration(Duration::from_secs(uptime)));
        buf.push(("Last boot", line));

        let cpus = sys.cpus();
        if !cpus.is_empty() {
            buf.push(("CPU", cpus[0].name().to_string()));

            let load = sys.load_average().fifteen;
            let line = format!("~{load:.2}%");
            buf.push(("Load average", line));
        }

        let used_mem = sys.used_memory() as u128;
        let total_mem = sys.total_memory() as u128;
        let percent = (used_mem as f64 / total_mem as f64) * 100.0;
        if percent != 0.0 {
            buf.push((
                "Memory",
                format!(
                    "{} / {} ({percent:.01} %)",
                    Byte::from_bytes(used_mem).get_appropriate_unit(true),
                    Byte::from_bytes(total_mem).get_appropriate_unit(true),
                ),
            ))
        }

        Self(buf)
    }

    //noinspection DuplicatedCode
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &String)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys;

    #[test]
    fn host_info() {
        let sys = sys();
        let host_info = HostInfo::new(&sys);
        assert_eq!(host_info.0.len(), 2)
    }

    #[test]
    fn user_info() {
        let sys = sys();
        let _ = UserInfo::new(&sys);
    }

    #[test]
    fn sys_info() {
        let sys = sys();
        let sys_info = SystemInfo::new(&sys);
        assert_eq!(sys_info.0.len(), 4)
    }

    #[test]
    fn disk_info() {
        let sys = sys();
        let _ = DiskInfo::from(sys.disks());
    }
}
