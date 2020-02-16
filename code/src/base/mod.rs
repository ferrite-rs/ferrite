
//! # Recur

//! The `base` module  defines the abstract interfaces for `session_rust`.
//!
//! - [`crate::base::Protocol`] - Protocol expressions in session types with
//!   translation to Rust channel types.

mod process;
mod processes;
mod session;
mod lens;
mod nat;
mod prism;
mod fix;

pub mod public;

pub use self::process::{
  Protocol
};

pub use self::processes::{
  Context,
  EmptyContext,
  AppendContext,
  Reversible
};

pub use self::session::{
  Session,
  PartialSession,
  create_partial_session,
  run_partial_session
};

pub use self::lens::{
  Slot,
  Empty,
  ContextLens,
};

pub use fix::*;
pub use nat::*;
pub use prism::*;
