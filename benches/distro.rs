use criterion::{criterion_group, criterion_main, Criterion};
use punfetch::Distro;
use std::fmt::Display;

const SAMPLES: [&str; 13] = [
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

fn search_match<S: Display>(str: S) {
    let distro = Distro::search(str);
    assert_ne!(distro, Distro::DEFAULT);
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("distro");
    group.significance_level(0.1);

    let len = SAMPLES.len();
    group.bench_function("search", move |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for i in 0..iters as usize {
                search_match(SAMPLES[i % len]);
            }
            start.elapsed()
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
