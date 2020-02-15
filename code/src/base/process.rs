use crate::base::nat::*;

/// A process / session type. This can be used as either input or output process.
pub trait Protocol
  : Send + 'static
{
  type Value : Sized + Send;
}

pub mod public {
  pub trait Protocol : super::Protocol {}
}

impl Protocol for Z {
  type Value = Z;
}

impl < N >
  Protocol for
  S < N >
where
  N : Nat
{
  type Value = S < N >;
}
