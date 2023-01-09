use clap::{arg, command};
use image::open;
use onefetch_image::get_best_backend;
use sysinfo::{CpuRefreshKind, ProcessRefreshKind, RefreshKind, System, SystemExt};
use term_size::dimensions;

use info::*;

mod info;

const LINUX: &str = r#"               _nnnn_
              dGGGGMMb
             @p~qp~~qMb
             M|@||@) M|
             @,----.JM|
            JS^\__/  qKL
           dZP        qKRb
          dZP          qKKb
         fZP            SMMb
         HZM            MMMM
         FqM            MMMM
       __| ".        |\dS"qML
       |    `.       | `' \Zq
      _)      \.___.,|     .'
      \____   )MMMMMP|   .'
           `-'       `--'
 "#;

fn main() {
    let args = command!()
        .args(vec![
            arg!(-i --image <PATH> "Image to use"),
            arg!(--"show-logo" <WHEN> "Show logo [always|auto|never]"),
        ])
        .about("")
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

    let sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new())
            .with_users_list()
            .with_processes(ProcessRefreshKind::new().with_user())
            .with_disks_list()
            .with_memory(),
    );

    let info = vec![
        user_info(&sys),
        os_info(&sys),
        disk_info(&sys, 30),
        sys_info(&sys),
        colors(),
    ]
    .into_iter()
    .flatten()
    .collect();

    let mut buf = String::new();

    if show_logo {
        if let Some((image, backend)) = args.get_one("image").map(|path: &String| {
            let image = open(path).expect("Failed to open image");
            let backend = get_best_backend().expect("Failed to find a backend");
            (image, backend)
        }) {
            buf.push_str(backend.add_image(info, &image, 32).unwrap().as_str());
        } else {
            let art: Vec<&str> = LINUX.lines().collect();
            let art_width = (art.iter().map(|s| s.len()).max().unwrap_or(0) + 6).max(art.len());
            for i in 0..art.len().max(info.len()) {
                let line = info.get(i).map(|i| i.to_string()).unwrap_or_default();
                let art = art.get(i).map(|i| i.to_string()).unwrap_or_default();
                buf.push_str(&format!("{art:<art_width$}{line}\n"));
            }
        }
    } else {
        for line in info {
            buf.push_str(&format!("{line}\n"));
        }
    }

    print!("{buf}");
}
