use clap::{arg, command};
use image::open;
use info::*;
use onefetch_image::get_best_backend;
use sysinfo::{System, SystemExt};

mod info;

fn main() {
    let args = command!()
        .arg(arg!(-i --image <PATH> "Image to use"))
        .about("")
        .get_matches();

    let mut sys = System::new_all();
    sys.refresh_all();

    let info = vec![user_info(), os_info(&sys), disk_info(&sys, 30), sys_info(&sys), colors()]
        .into_iter()
        .flatten()
        .collect();

    let mut buf = String::new();

    if let Some((image, backend)) = args.get_one("image").map(|path: &String| {
        let image = open(path).expect("Failed to open image");
        let backend = get_best_backend().expect("Failed to find a backend");
        (image, backend)
    }) {
        buf.push_str(backend.add_image(info, &image, 32).unwrap().as_str());
    } else {
        for line in info {
            buf.push_str(format!("{line}\n").as_str());
        }
    }

    print!("{buf}");
}
