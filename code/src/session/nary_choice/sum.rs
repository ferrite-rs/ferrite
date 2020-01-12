
pub use crate::base::*;
pub use crate::process::nary_choice::*;

pub trait ExternalSum < I >
  : ProcessSum
where
  I : Processes
{
  type SessionSum;
}

impl
  < I, P >
  ExternalSum < I >
  for P
where
  P : Process,
  I : Processes
{
  type SessionSum =
    PartialSession < I, P >;
}

impl
  < I, P, R >
  ExternalSum < I >
  for Sum < P, R >
where
  P : Process,
  R : ExternalSum < I >,
  I : Processes,
{
  type SessionSum =
    Sum <
      PartialSession <
        I,
        P
      >,
      R :: SessionSum
    >;
}
