
//! # Recur

//! The `base` module  defines the abstract interfaces for `session_rust`.
//!
//! - [`crate::base::Protocol`] - Protocol expressions in session types with
//!   translation to Rust channel types.

mod protocol;
mod context;
mod session;
mod lens;
mod nat;
mod prism;
mod fix;
mod core;

pub mod public;

pub use self::core::{
  Refl
};

pub use self::protocol::{
  Protocol
};

pub use self::context::{
  Context,
  EmptyContext,
  AppendContext,
  Reversible
};

pub use self::session::{
  Session,
  PartialSession,
  unsafe_create_session,
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
