use std::{path::PathBuf, process::exit, str::FromStr};

use clap::{arg, command, Parser, ValueEnum};
use image::open;
use owo_colors::DynColors;
use sysinfo::SystemExt;
use term_size::dimensions;

use punfetch::{info::*, Distro, Printer};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Distribution to search art for (e.g. "Manjaro" or "Ubuntu")
    #[arg(long)]
    distro: Option<String>,
    /// Image to display in place of the distro art
    #[arg(short, long)]
    image: Option<PathBuf>,
    /// Show the logo
    #[arg(long, value_name = "WHEN", default_value = "always")]
    #[clap(value_enum)]
    show_logo: ShowLogo,
    /// Color mode to use
    #[arg(long, value_name = "MODE", default_value = "hex")]
    #[clap(value_enum)]
    color_mode: ColorMode,
    /// Text color to use. Accepts ansi or hex color codes
    #[arg(long, value_name = "COLOR")]
    color: Option<String>,
}

#[derive(Default, Clone, ValueEnum)]
enum ShowLogo {
    #[default]
    Always,
    Never,
    Auto,
}

impl ShowLogo {
    fn should_show(&self) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => dimensions().map(|(w, _)| w > 95).unwrap_or(false),
        }
    }
}

#[derive(Default, Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum ColorMode {
    #[default]
    HEX,
    ANSI,
    NONE,
}

impl ColorMode {
    fn mode(&self) -> Option<bool> {
        match self {
            Self::HEX => Some(true),
            Self::ANSI => Some(false),
            Self::NONE => None,
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut renderer = Printer::new(args.color_mode.mode());
    if let Some(color) = args.color {
        if let Ok(color) = DynColors::from_str(&color) {
            renderer.color = Some(color);
        } else {
            eprintln!("Invalid color: {color}");
            exit(1);
        }
    }

    let sys = sys();
    let host_info = HostInfo::new(&sys); // need the distro!

    if args.show_logo.should_show() {
        if let Some(path) = args.image {
            match open(path) {
                Ok(image) => renderer.with_image(image),
                Err(e) => eprintln!("Error opening image: {e}"),
            }
        } else {
            let str = args.distro.unwrap_or(host_info.distro.clone());
            renderer.with_distro(Distro::search(str));
        }
    }

    renderer.with_info(UserInfo::new(&sys));
    renderer.with_info(host_info);
    renderer.with_info(DiskInfo::from(sys.disks()));
    renderer.with_info(SystemInfo::new(&sys));

    renderer.render()
}
