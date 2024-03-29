# Ferrite - Session Types DSL for Rust

[![Crates.io][crates-badge]][crates-url]
[![Documentation][doc-badge]][doc-url]
[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/ferrite-session.svg
[crates-url]: https://crates.io/crates/ferrite-session
[doc-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[doc-url]: https://ferrite-rs.github.io/ferrite-doc/ferrite_session/
[license-badge]: https://img.shields.io/crates/l/ferrite-session.svg
[license-url]: https://github.com/ferrite-rs/ferrite/blob/master/LICENSE
[actions-badge]: https://github.com/ferrite-rs/ferrite/workflows/Cargo%20Tests/badge.svg
[actions-url]: https://github.com/ferrite-rs/ferrite/actions

## Overview

Ferrite is a DSL for writing session type programs in Rust.
This is an ongoing research work by [Soares Chen](https://maybevoid.com/)
and [Stephanie Balzer](http://www.cs.cmu.edu/~balzers/), and
[Bernardo Toninho](http://ctp.di.fct.unl.pt/~btoninho/) to implement
session types in Rust.

## Documentation

- The paper _Ferrite: A Judgmental Embedding of Session Types in Rust_
  is published at [ECOOP 2022](https://doi.org/10.4230/LIPIcs.ECOOP.2022.22).

- A technical report for Ferrite is currently available at
[Arxiv](https://arxiv.org/abs/2009.13619).

- A work-in-progress documentation for Ferrite is available as a
[book](http://ferrite-rs.github.io/ferrite-book/).

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

## Acknowledgement

This material is based upon work supported by the National Science Foundation under Grant No. CCF-1718267.
Any opinions, findings, and conclusions or recommendations expressed in this material are those of the author(s)
and do not necessarily reflect the views of the National Science Foundation.
