use std::{ffi::OsStr, io, process::Command, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};

fn ascii() -> u64 {
    exec("./target/debug/punfetch", vec![]).unwrap()
}

fn text() -> u64 {
    exec("./target/debug/punfetch", vec!["--show-logo", "never"]).unwrap()
}

fn _image() -> u64 {
    exec(
        "./target/release/punfetch",
        vec!["-i", "./benches/term.png"],
    )
    .unwrap()
}

fn bench(c: &mut Criterion) {
    let mut cli = c.benchmark_group("cli");
    cli.significance_level(0.1)
        .measurement_time(Duration::from_secs(34));
    cli.bench_function("ascii", |b| b.iter(ascii));
    cli.bench_function("text", |b| b.iter(text));
    // todo: disabled for now, look into why it wont run
    // group.bench_function("image", |b| b.iter(image));
    cli.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

fn exec<S: AsRef<OsStr>>(bin: S, args: Vec<S>) -> io::Result<u64> {
    let output = Command::new(bin).args(args).output()?;
    Ok(output.status.code().unwrap_or(1) as u64)
}
