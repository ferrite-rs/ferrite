# Ferrite - Session Types DSL for Rust

## Overview

Ferrite is a DSL for writing session type programs in Rust.

## Documentation

A work-in-progress documentation for Ferrite is available as a
[book](http://ferrite-rs.github.io/ferrite-book/).

A draft technical report for Ferrite is currently available at
[Arxiv](https://arxiv.org/abs/2009.13619).

## Build Instructions

```bash
cargo build
```

## Running Demo

A number of demo executables are available in the [`src/bin`](src/bin) directory.
To run a particular demo, use `cargo run` with the name of the demo file.
For example:

```bash
RUST_LOG=info cargo run --bin hello
RUST_LOG=info cargo run --bin shared
```
