# Ferrite - Session Types DSL for Rust

## Overview

Ferrite is a DSL for writing session type programs in Rust.

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
