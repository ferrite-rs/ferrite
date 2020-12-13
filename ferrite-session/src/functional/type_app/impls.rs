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

  fn get_applied_borrow < 'a >
    ( &'a self )
    -> &'a F::Applied
  where
    F: TypeApp < A >
  { self }

  fn get_applied_borrow_mut < 'a >
    ( &'a mut self )
    -> &'a mut F::Applied
  where
    F: TypeApp < A >
  { self }
}

impl < T, F, A, K >
  TypeAppWitness < F, A, K >
  for ()
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  K: 'static,
  F: TypeApp < A, Applied=T >,
{
  fn with_applied
    ( &self,
      cont: Box < dyn TypeAppCont < F, A, K > >
    ) -> K
  {
    cont.on_type_app()
  }

  fn clone_witness
    ( &self )
    -> Box < dyn TypeAppWitness < F, A, K > >
  {
    Box::new( () )
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
