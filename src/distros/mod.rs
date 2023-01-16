//! Supported distributions. Base enum is generated from the `distros.yaml` file

use std::fmt::Display;

use onefetch_ascii::AsciiArt;

include!(concat!(env!("OUT_DIR"), "/distros.rs"));

impl Distro {
    /// Find a distro from a string
    pub fn search<S: Display>(str: S) -> Self {
        // strip special characters and 'linux' from input
        let reg = regex::Regex::new(r"[\s_\-\./!@]").unwrap();
        let str = str.to_string(); // increase lifetime
        let str = reg
            .replace_all(&str, "")
            .to_ascii_lowercase()
            .replace("linux", "");

        // search for distro
        let m = Self::regex().matches(&str);
        if let Some(m) = m.into_iter().last() {
            m.into()
        } else {
            Distro::DEFAULT
        }
    }

    /// Build ascii art from the inner template and colors
    pub fn ascii<'a>(&self, color_mode: Option<bool>) -> AsciiArt<'a> {
        let colors = self.colors(color_mode);
        let bold = !colors.is_empty();
        AsciiArt::new(self.template(), colors.leak(), bold)
    }

    /// Get the primary color for the distro
    pub fn color(&self) -> DynColors {
        self.colors(None)
            .get(0)
            .unwrap_or(&DynColors::Ansi(AnsiColors::Default))
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Distro;

    /// Ensure some common distros with extra data in them will match correctly
    #[test]
    fn search() {
        const DISTROS: [(Distro, &str); 10] = [
            (Distro::MANJARO, "Manjaro Linux"),
            (Distro::ARCH, "Arch Linux"),
            (Distro::UBUNTU, "Ubuntu 20.04.1 LTS"),
            (Distro::DEBIAN, "Debian GNU/Linux 10 (buster)"),
            (Distro::GENTOO, "Gentoo Base System release 2.7"),
            (Distro::FEDORA, "Fedora 33 (Thirty Three)"),
            (Distro::CENTOS, "CentOS Linux 8 (Core)"),
            (Distro::OPENSUSELEAP, "openSUSE Leap 15.2"),
            (Distro::VOID, "Void 5.8.14_1 x86_64"),
            (Distro::DEFAULT, "Unknown"),
        ];

        for (distro, name) in DISTROS.iter() {
            assert_eq!(*distro, Distro::search(name));
        }
    }
}
