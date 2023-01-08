<img src="https://user-images.githubusercontent.com/8976745/211187132-1a5e959b-d3a4-4c84-84c1-f8bd5463a30e.png" height="300px">

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ozwaldorf/punfetch/rust.yml?label=CI&style=for-the-badge)](https://github.com/ozwaldorf/punfetch/actions/workflows/rust.yml)
[![crate](https://img.shields.io/crates/v/punfetch?style=for-the-badge)](https://crates.io/crates/onefetch)
[![downloads](https://img.shields.io/crates/d/punfetch?style=for-the-badge)](https://crates.io/crates/punfetch)
[![license](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](./LICENSE)
[![issues](https://img.shields.io/github/issues-raw/ozwaldorf/punfetch?style=for-the-badge)](https://github.com/ozwaldorf/punfetch/issues)

A blazingly fast sysinfo fetcher designed to match [onefetch's](https://github.com/o2sh/onefetch) formatting.

---

![image](https://user-images.githubusercontent.com/8976745/211184085-b4fb05d5-b31b-4d85-9320-1e2060d6db6d.png)

## Installation

### Prerequisites

- Rust

### Install from crates.io

```bash
cargo install punfetch
```

### Build from source

```bash
git clone https://github.com/ozwaldorf/punfetch.git
cd punfetch
make install
```

## Usage

```man
Usage: punfetch [OPTIONS]

Options:
  -i, --image <PATH>      Image to use
      --show-logo <WHEN>  Show logo [always|auto|never]
  -h, --help              Print help information
  -V, --version           Print version information
```

## Contributing

This project follows [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).

[![contrib.rocks](https://contrib.rocks/image?repo=ozwaldorf/punfetch)](https://github.com/ozwaldorf/punfetch/graphs/contributors)
