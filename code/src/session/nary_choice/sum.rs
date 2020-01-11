
pub use crate::base::*;
pub use crate::process::nary_choice::*;

pub trait ExternalSum < I >
  : ProcessSum
where
  I : Processes
{
  type SessionSum;
}

pub trait InternalSum  < S, D, P, Lens >
  : ProcessSum
where
  P : Process,
  S : Processes,
  D : Processes,
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

// impl
//   < Lens, I, T, D, P, Q, R >
//   InternalSum < I, D, P, Lens >
//   for Sum < Q, R >
// where
//   P : Process,
//   Q : Process,
//   R : InternalSum < I, D, P, Lens >,
//   I : Processes,
//   D : Processes,
//   Lens :
//     ProcessLens <
//       I, T, D, P, Q
//     >,
// {
//   type SessionSum = ();
// }