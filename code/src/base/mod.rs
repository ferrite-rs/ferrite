
//! # Base

//! The `base` module  defines the abstract interfaces for `session_rust`.
//!
//! - [`crate::base::Process`] - Process expressions in session types with
//!   translation to Rust channel types.

mod process;
mod processes;
mod session;
mod lens;
mod util;

pub use self::process::{ Process };
pub use self::processes::*;
pub use self::session::*;
pub use self::lens::*;
pub use self::util::*;
