use criterion::{criterion_group, criterion_main, Criterion};
use std::ffi::OsStr;
use std::io;
use std::process::Command;
use std::time::Duration;

fn exec<S: AsRef<OsStr>>(bin: S, args: Vec<S>) -> io::Result<u64> {
    let output = Command::new(bin).args(args).output()?;
    Ok(output.status.code().unwrap_or(1) as u64)
}

fn ascii_cli() -> u64 {
    exec("./target/release/punfetch", vec![]).unwrap()
}

fn text_cli() -> u64 {
    exec("./target/release/punfetch", vec!["--show-logo", "never"]).unwrap()
}

fn _image_cli() -> u64 {
    exec(
        "./target/release/punfetch",
        vec!["-i", "./benches/term.png"],
    )
    .unwrap()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("cli");
    group
        .significance_level(0.1)
        .measurement_time(Duration::from_secs(30));
    group.bench_function("ascii", |b| b.iter(ascii_cli));
    group.bench_function("text", |b| b.iter(text_cli));
    // todo: disabled for now, look into why it wont run
    // group.bench_function("image", |b| b.iter(image_cli));

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
