# punfetch

A blazingly fast sysinfo fetcher designed to match [onefetch](https://github.com/o2sh/onefetch)'s formatting.

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
