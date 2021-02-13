# qboot-rs

- [qboot-rs](#qboot-rs)
  - [What's this?](#whats-this)
  - [Requirements](#requirements)
  - [Setup](#setup)
  - [About license](#about-license)

## What's this?

Rewrite [qboot](https://github.com/selpoG/qboot) in `Rust`.

## Requirements

- [bindgen](https://github.com/rust-lang/rust-bindgen)
- [gmp](https://gmplib.org/) (or [mpir](https://github.com/BrianGladman/mpir.git) on Windows)
- [mpfr](http://mpfr.org/) (we recommend [this repository](https://github.com/BrianGladman/mpfr.git) on Windows)

## Setup

`<ROOT>` is the root directory of this repository.

1. Copy `gmp.h` and `mpfr.h` to `<ROOT>/src/mp/c/`.
2. Copy `libgmp.a` and `libmpfr.a` to `.`.
3. Install `bindgen`: `cargo install bindgen`.
4. Generate rust bindings by:
   1. `bindgen src/mp/c/gmp.h > src/mp/bindings/gmp.rs`
   2. `bindgen src/mp/c/mpfr.h > src/mp/bindings/mpfr.rs`
5. `RUSTFLAGS="-lgmp -lmpfr" cargo run`

On Windows, use `mpir` instead of `gmp`, and `xxx.lib` instead of `libxxx.a`.
If you use `Powershell`, you can set environment variables by `$env:RUSTFLAGS = "-lmpir -lmpfr"`.

## About license

The source code in this repository is under [LICENSE](/LICENSE),
but if you link `gmp` or `mpfr`, you need to follow their licenses.

このリポジトリに含まれるソースコードには [LICENSE](/LICENSE) が適用されますが,
`gmp` や `mpfr` をリンクする場合にはそれらのライセンスに従う必要があります.
