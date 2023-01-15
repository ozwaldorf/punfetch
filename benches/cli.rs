use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

mod cli {
    use std::{ffi::OsStr, io, process::Command};

    fn exec<S: AsRef<OsStr>>(bin: S, args: Vec<S>) -> io::Result<u64> {
        let output = Command::new(bin).args(args).output()?;
        Ok(output.status.code().unwrap_or(1) as u64)
    }

    pub(crate) fn ascii_cli() -> u64 {
        exec("./target/debug/punfetch", vec![]).unwrap()
    }

    pub(crate) fn text_cli() -> u64 {
        exec("./target/debug/punfetch", vec!["--show-logo", "never"]).unwrap()
    }

    pub(crate) fn _image_cli() -> u64 {
        exec(
            "./target/release/punfetch",
            vec!["-i", "./benches/term.png"],
        )
        .unwrap()
    }
}

fn bench(c: &mut Criterion) {
    let mut cli = c.benchmark_group("cli");
    cli.significance_level(0.1)
        .measurement_time(Duration::from_secs(30));
    cli.bench_function("ascii", |b| b.iter(cli::ascii_cli));
    cli.bench_function("text", |b| b.iter(cli::text_cli));
    // todo: disabled for now, look into why it wont run
    // group.bench_function("image", |b| b.iter(image_cli));
    cli.finish();

    let mut distro = c.benchmark_group("distro");
    distro.significance_level(0.1);
    let samples = vec![
        "Arch Linux",
        "Debian GNU/Linux",
        "Fedora Linux",
        "Gentoo Linux",
        "Linux Mint",
        "Manjaro Linux",
        "openSUSE Leap",
        "openSUSE Tumbleweed",
        "Pop!_OS",
        "Ubuntu",
        "Void Linux",
        "Solus",
        "elementary OS",
    ];
    let len = samples.len();
    distro.bench_function("search", move |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for i in 0..iters as usize {
                distros::search_match(samples.get(i % len).unwrap());
            }
            start.elapsed()
        })
    });
    distro.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
