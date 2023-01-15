use std::fmt::Display;

include!(concat!(env!("OUT_DIR"), "/distros.rs"));

impl Distro {
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
        if let Some(m) = m.into_iter().next() {
            m.into()
        } else {
            Distro::DEFAULT
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Distro;

    /// Ensure some common distros with extra data in them will match correctly
    #[test]
    fn test_search() {
        let manjaro = Distro::search("Manjaro Linux");
        assert_eq!(manjaro, Distro::MANJARO);

        let arch = Distro::search("Arch Linux");
        assert_eq!(arch, Distro::ARCH);

        let ubuntu = Distro::search("Ubuntu 20.04.1 LTS");
        assert_eq!(ubuntu, Distro::UBUNTU);

        let debian = Distro::search("Debian GNU/Linux 10 (buster)");
        assert_eq!(debian, Distro::DEBIAN);

        let fedora = Distro::search("Fedora 33 (Thirty Three)");
        assert_eq!(fedora, Distro::FEDORA);

        let centos = Distro::search("CentOS Linux 8 (Core)");
        assert_eq!(centos, Distro::CENTOS);

        let opensuse = Distro::search("openSUSE Leap 15.2");
        assert_eq!(opensuse, Distro::OPENSUSE);

        let gentoo = Distro::search("Gentoo Base System release 2.7");
        assert_eq!(gentoo, Distro::GENTOO);

        let void = Distro::search("Void 5.8.14_1 x86_64 MUSL");
        assert_eq!(void, Distro::VOID);
    }
}
