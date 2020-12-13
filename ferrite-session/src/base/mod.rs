
//! The `base` module  defines the abstract interfaces for `session_rust`.
//!
//! - [`crate::base::Protocol`] - Protocol expressions in session types with
//!   translation to Rust channel types.

mod protocol;
mod context;
mod session;
mod lens;
mod fix;

pub mod public;

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
  unsafe_run_session
};

pub use self::lens::{
  Slot,
  Empty,
  ContextLens,
};

pub use fix::*;
