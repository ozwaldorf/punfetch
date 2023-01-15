use std::fmt::Display;

use colored::{Color, Colorize};
use image::DynamicImage;
use onefetch_image::get_best_backend;

use crate::{
    distros::Distro,
    info::{DiskInfo, HostInfo, SystemInfo, UserInfo},
};

pub struct Renderer {
    pub os: String,
    pub color: Option<Color>,
    pub width: usize,
    pub lines: Vec<String>,
    pub distro: Option<Distro>,
    pub user_info: Option<UserInfo>,
    pub host_info: Option<HostInfo>,
    pub disk_info: Option<DiskInfo>,
    pub sys_info: Option<SystemInfo>,
    pub image: Option<DynamicImage>,
}

impl Renderer {
    fn colors() -> String {
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
        )
    }

    pub fn new(os: String, show_logo: bool) -> Self {
        let distro = show_logo.then(|| Distro::search(os.clone()));

        Self {
            os,
            distro,
            color: Some(Color::Green),
            width: 30,
            lines: Vec::new(),
            user_info: None,
            sys_info: None,
            disk_info: None,
            host_info: None,
            image: None,
        }
    }

    #[inline]
    pub fn with_image(&mut self, image: DynamicImage) {
        self.image = Some(image);
    }

    #[inline]
    pub fn with_user_info<I: Into<UserInfo>>(&mut self, info: I) {
        self.user_info = Some(info.into());
    }

    #[inline]
    pub fn with_host_info<I: Into<HostInfo>>(&mut self, info: I) {
        self.host_info = Some(info.into());
    }

    #[inline]
    pub fn with_disk_info<I: Into<DiskInfo>>(&mut self, disks: I) {
        self.disk_info = Some(disks.into())
    }

    #[inline]
    pub fn with_sys_info<I: Into<SystemInfo>>(&mut self, sys_info: I) {
        self.sys_info = Some(sys_info.into());
    }

    fn render_disks(&self) -> Option<Vec<String>> {
        self.disk_info.as_ref().map(|info| {
            let mut bufs = vec![format!("{}: ", "Disks".green().bold())];

            let colors = vec![
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::White,
            ];

            let max_width = self.width as f64;
            let total = info.total;
            let mut remainder = max_width as usize;
            for (i, (name, used)) in info.keys.iter().zip(info.values.to_owned()).enumerate() {
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
        })
    }

    #[inline]
    pub fn render(&mut self) {
        if let Some(info) = &self.user_info {
            let (user, host) = self
                .color
                .map(|c| (info.user.color(c).bold(), info.host.color(c).bold()))
                .unwrap_or((info.user.bold(), info.host.bold()));
            let len = info.user.len() + info.host.len() + 3;
            self.lines.push(format!("{user} ~ {host}"));
            self.lines.push("-".repeat(len));
        }

        if let Some(info) = &mut self.host_info {
            info.0.push_front(("OS", self.os.clone()));
            push_fmt(&mut self.lines, info.iter(), self.color);
        }

        if let Some(disks) = self.render_disks() {
            self.lines.extend(disks)
        }

        if let Some(info) = &self.sys_info {
            push_fmt(&mut self.lines, info.iter(), self.color);
        }

        self.lines.push("".to_string());
        self.lines.push(Renderer::colors());

        // build the final buffer
        let mut buf = String::new();
        if let Some((image, backend)) = self
            .image
            .as_ref()
            .and_then(|img| get_best_backend().map(|backend| (img, backend)))
        {
            buf.push_str(
                backend
                    .add_image(self.lines.clone(), image, 32)
                    .unwrap()
                    .as_str(),
            );
        } else if let Some(distro) = &self.distro {
            let art: Vec<&str> = distro.ascii_stripped().lines().collect();
            let art_width = distro.width();
            for i in 0..art.len().max(self.lines.len()) {
                let line = self.lines.get(i).map(|i| i.to_string()).unwrap_or_default();
                let art = art.get(i).map(|i| i.to_string()).unwrap_or_default();
                buf.push_str(&format!("  {art:<art_width$}  {line}\n"));
            }
        } else {
            for line in self.lines.as_slice() {
                buf.push_str(line);
                buf.push('\n');
            }
        }

        print!("{buf}")
    }
}

fn push_fmt<V: Display>(
    buf: &mut Vec<String>,
    iter: impl Iterator<Item = (&'static str, V)>,
    color: Option<Color>,
) {
    if let Some(color) = color {
        buf.extend(iter.map(|(k, v)| format!("{}: {v}", k.bold().color(color),)));
    } else {
        buf.extend(iter.map(|(k, v)| format!("{}: {}", k.bold(), v)));
    }
}
