use clap::{arg, command};
use image::open;
use sysinfo::SystemExt;
use term_size::dimensions;

use libpunfetch::{
    info::{sys, HostInfo, SystemInfo, UserInfo},
    render::Renderer,
};

fn main() {
    let args = command!()
        .args(vec![
            arg!(-i --image <PATH> "Image to use"),
            arg!(--"show-logo" <WHEN> "Show logo [always|auto|never]"),
        ])
        .get_matches();

    let show_logo = match args
        .get_one::<String>("show-logo")
        .unwrap_or(&"always".to_string())
        .as_str()
    {
        "always" => true,
        "auto" => dimensions().unwrap_or((0, 0)).0 >= 95,
        "never" => false,
        _ => panic!("Invalid value for --show-logo. Valid values are: always, never, auto"),
    };

    let sys = sys();

    let os = sys.name().unwrap_or(sys.distribution_id());
    let mut renderer = Renderer::new(os, show_logo);
    renderer.with_user_info(UserInfo::new(&sys));
    renderer.with_host_info(HostInfo::new(&sys));
    renderer.with_disk_info(sys.disks());
    renderer.with_sys_info(SystemInfo::new(&sys));
    if let Some(path) = args.get_one::<String>("image") {
        match open(path) {
            Ok(image) => renderer.with_image(image),
            Err(e) => eprintln!("Error opening image: {e}"),
        }
    }

    renderer.render()
}
