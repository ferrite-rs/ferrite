use crate::base::nat::*;

/// A process / session type. This can be used as either input or output process.
pub trait Protocol : Send + 'static
{ }

pub mod public {
  pub trait Protocol : super::Protocol {}
}

impl < A >
  public::Protocol
  for A
where
  A : Protocol
{}

impl Protocol for Z { }

impl < N >
  Protocol for
  S < N >
where
  N : Protocol
{ }
