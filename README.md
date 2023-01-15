<header>
    <p align="center">
        <img alt="logo" src="https://user-images.githubusercontent.com/8976745/211187132-1a5e959b-d3a4-4c84-84c1-f8bd5463a30e.png" width="50%"/>
    </p>
    <p align="center">
        <a href="https://crates.io/crates/punfetch"><img alt="crate" src="https://img.shields.io/crates/v/punfetch?style=for-the-badge" /></a>
        <a href="https://crates.io/crates/punfetch"><img alt="downloads" src="https://img.shields.io/crates/d/punfetch?style=for-the-badge" /></a>
        <a href="./LICENSE"><img alt="license" src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge" /></a>
        <a href="https://github.com/ozwaldorf/punfetch/actions/workflows/rust.yml"><img alt="ci" src="https://img.shields.io/github/actions/workflow/status/ozwaldorf/punfetch/rust.yml?label=CI&style=for-the-badge" /></a>
        <a href="https://github.com/ozwaldorf/punfetch/actions/workflows/publish.yml"><img alt="publish" src="https://img.shields.io/github/actions/workflow/status/ozwaldorf/punfetch/publish.yml?label=Publish&style=for-the-badge" /></a>
    </p>
    <p align="center">
        A blazingly fast system fetch program to pair with <a href="https://github.com/o2sh/onefetch">onefetch</a>
    </p>
    <hr>
    <p align="center">
        <img alt="preview" width="80%" src="https://user-images.githubusercontent.com/8976745/211231336-194d6836-154b-4189-beac-b022d3056504.png"/> 
    </p>
</header>

---

# Installation

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

# Usage

```man
$ punfetch -h
Usage: punfetch [OPTIONS]

Options:
  -i, --image <PATH>      Image to use
      --show-logo <WHEN>  Show logo [always|auto|never]
  -h, --help              Print help information
  -V, --version           Print version information
```

# Todo

- Improve sysinfo query time
- Ascii colors ([see this](https://github.com/dylanaraps/neofetch/blob/master/neofetch))
- GPU info
- Smart colors from image
- Full onefetch formatting feature parity

# Contributing

This project follows [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)!

## Adding a new distribution

New distributions can easily be supported by adding a new line to the [distros.yaml](distros.yaml) file. A simple example entry might look like:

```yaml
"Examplo Linux":
  ascii: |
    {0}         _nnnn_
    {0}        dGGGGMMb
    {0}       @p~qp~~qMb
    {0}       M|@||@) M|
    {0}       @,----.JM|
    {0}      JS^\__/  qKL
    {0}     dZP        qKRb
    {0}    dZP          qKKb
    {0}   fZP            SMMb
    {0}   HZM            MMMM
    {0}   FqM            MMMM
    {0} __| ".        |\dS"qML
    {0} |    `.       | `' \Zq
    {0}_)      \.___.,|     .'
    {0}\____   )MMMMMP|   .'
    {0}     `-'       `--'
```

If there are multiple patterns that could be used to identify the distribution, or the title of the distribution is lengthy, a custom pattern can be specified:

```yaml
"Examplo Linux":
  regex: "{examplo,oldname1,oldname2}"
  ascii: |
    ...
```

> Regex patterns should be simple, lowercase, and `A-z0-9` only.


<footer>
    <h2 align="center">Contributors ❤️</h2>
    <p align="center">
        <a href="https://github.com/ozwaldorf/punfetch/graphs/contributors"><img alt="contrib.rocks" src="https://contrib.rocks/image?repo=ozwaldorf/punfetch"/></a>
    </p>
    <p align="center">(<a href="https://contrib.rocks/preview?repo=ozwaldorf%2Fpunfetch">contrib.rocks</a>)</p>
</footer>
