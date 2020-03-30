use crate::base::nat::*;

/// A process / session type. This can be used as either input or output process.
pub trait Protocol : Send + 'static
{
  type Payload : Sized + Send;
}

pub mod public {
  pub trait Protocol : super::Protocol {}
}

impl Protocol for Z {
  type Payload = Z;
}

impl < N >
  Protocol for
  S < N >
where
  N : Protocol
{
  type Payload = S < N :: Payload >;
}
