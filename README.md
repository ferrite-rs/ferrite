# Ferrite - Session Type EDSL for Rust

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