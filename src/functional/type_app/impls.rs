use super::traits::*;
use super::structs::*;

impl < T, F, A >
  HasTypeApp < F, A >
  for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  F: TypeApp < A, Applied=T >
{
  fn get_applied (self: Box < T >) -> Box < T >
  { self }
}

impl < T, F, A, K >
  TypeAppWitness < F, A, K >
  for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  K: 'static,
  F: TypeApp < A, Applied=T >,
{
  fn with_applied
    ( self: Box < Self >,
      cont: Box < dyn TypeAppWitnessCont < F, A, K > >
    ) -> K
  {
    cont.on_witness(self)
  }
}

impl TyCon for () {}

impl < X > TyCon for Const < X >
where
  X: 'static
{}

impl < A > TypeApp < A >
  for ()
where
  A: 'static
{
  type Applied = ();
}

impl < X, A > TypeApp < A >
  for Const < X >
where
  A: 'static,
  X: Send + 'static,
{
  type Applied = X;
}
