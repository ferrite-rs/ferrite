//! The `base` module  defines the abstract interfaces for `session_rust`.
//!
//! - [`crate::base::Protocol`] - Protocol expressions in session types with
//!   translation to Rust channel types.

mod channel;
mod context;
mod fix;
mod lens;
mod protocol;
mod session;

pub mod public;

pub use fix::*;

pub use self::{
  channel::*,
  context::{AppendContext, Context, EmptyContext, Reversible},
  lens::{ContextLens, Empty, Slot},
  protocol::Protocol,
  session::{
    unsafe_create_session,
    unsafe_run_session,
    PartialSession,
    Session,
  },
};
