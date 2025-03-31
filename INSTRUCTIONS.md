# Getting Started

## Prerequisites

- Ndless toolchain installed and added to path
- Rustup installed
- Latest Rust Nightly installed (nightly-2020-05-07 works, nightly-2025-03-05 does too)
- Unix-like (tested on Linux, and Mac, Cygwin will work as well)

Complete install script:

```bash
curl https://sh.rustup.rs -sSf | sh # skip if rustup already installed
rustup install nightly # skip if nightly already installed
cargo install cargo-ndless
```

Get started by running

```bash
cargo +nightly ndless build
```

to start development. Your .tns file will be available in
`target/armv5te-nspire-eabi/debug/ntradeadvance.tns`.

When you're ready to release your application,
**don't forget to compile in release mode** with

```bash
cargo +nightly ndless build -- --release
```

Your .tns file will be available in
`target/armv5te-nspire-eabi/release/ntradeadvance.tns`.

If you have the Firebird emulator installed, you can also send the compiled
binary straight to it. Just run:

```bash
cargo +nightly ndless run
cargo +nightly ndless run -- --release
```

## Building

### Ti-Nspire

```sh
cargo +nightly ndless build -- --no-default-features --features calculator-build --release
```

### Mac (Intel)
```sh
rustup target add x86_64-apple-darwin
cargo build --target x86_64-apple-darwin --release --no-default-features --features desktop --bin ntradeadvance-mac
```

### Mac (Apple Silicon)
```sh
rustup target add aarch64-apple-darwin
cargo build --target aarch64-apple-darwin --release --no-default-features --features desktop --bin ntradeadvance-mac
```

### Linux

Install [cross](https://github.com/cross-rs/cross) & [docker](https://docs.docker.com/get-started/get-docker/).

```sh
cross build --target aarch64-unknown-linux-gnu --release --no-default-features --features desktop --bin ntradeadvance-linux
```

### Windows

Install [cross](https://github.com/cross-rs/cross) & [docker](https://docs.docker.com/get-started/get-docker/).

```sh
cross build --target x86_64-pc-windows-gnu --release --no-default-features --features desktop --bin ntradeadvance-windows
```
