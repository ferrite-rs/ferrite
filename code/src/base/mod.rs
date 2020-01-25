
//! # Base

//! The `base` module  defines the abstract interfaces for `session_rust`.
//!
//! - [`crate::base::Process`] - Process expressions in session types with
//!   translation to Rust channel types.

mod process;
mod processes;
mod session;
mod lens;
mod nat;
mod prism;

pub mod public;

pub use self::process::{
  Process
};

pub use self::processes::{
  Processes,
  EmptyList,
  Appendable,
  Reversible
};

pub use self::session::{
  Session,
  PartialSession,
  create_partial_session,
  run_partial_session
};

pub use self::lens::{
  ProcessNode,
  Inactive,
  ProcessLens,
};

pub use nat::*;
pub use prism::*;