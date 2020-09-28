# Ferrite - Session Types DSL for Rust

## Overview

Ferrite is a DSL for writing session type programs in Rust.
This is an ongoing research work by [Soares Chen](https://maybevoid.com/)
and [Stephanie Balzer](http://www.cs.cmu.edu/~balzers/) to implement
session types in Rust.

A draft technical report for Ferrite is currently available
[here](https://maybevoid.com/pdf/ferrite-draft-2020-09.pdf)

## Build Instructions

The library code requires nightly version of Rust to be compiled.
You can use `rustup` to install Rust nightly as follows:

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup default nightly
```

After that, the library can be build with `cargo` as follows:

```bash
cargo build
```

## Runnnig Demo

A number of demo executables are available in the [`src/bin`](src/bin) directory.
To run a particular demo, use `cargo run` with the name of the demo file.
For example:

```bash
cargo run --bin hello
cargo run --bin shared
```

## Code Organization

  - [`src/base`](src/base) - Core constructs for Ferrite
  - [`src/protocol`](src/protocol) - Type definitions for session types
  - [`src/context`](src/context) - Type definitions for linear context
  - [`src/session`](src/session) - Term constructors
  - [`src/session`](src/shared) - Shared session types
  - [`src/public.rs`](src/public.rs) - Public API exposed by Ferrite
  - [`src/bin`](src/bin) - Demo executables

## Acknowledgement

This material is based upon work supported by the National Science Foundation under Grant No. CCF-1718267.
Any opinions, findings, and conclusions or recommendations expressed in this material are those of the author(s)
and do not necessarily reflect the views of the National Science Foundation.
