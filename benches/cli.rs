use std::{ffi::OsStr, io, process::Command, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};

fn exec<S: AsRef<OsStr>>(bin: S, args: Vec<S>) -> io::Result<u64> {
    let output = Command::new(bin).args(args).output()?;
    Ok(output.status.code().unwrap_or(1) as u64)
}

fn bench(c: &mut Criterion) {
    let mut cli = c.benchmark_group("cli");
    cli.significance_level(0.1)
        .measurement_time(Duration::from_secs(34));

    cli.bench_function("ascii", |b| {
        b.iter(|| exec("./target/debug/punfetch", vec![]).unwrap())
    });

    cli.bench_function("text", |b| {
        b.iter(|| exec("./target/debug/punfetch", vec!["--show-logo", "never"]).unwrap())
    });

    cli.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
