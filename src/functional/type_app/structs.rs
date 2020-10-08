use std::any::Any;
use std::marker::PhantomData;

use super::traits::*;
pub struct Applied < F, A >
{
  pub applied:
    Box < dyn TypeAppWitness <
      F, A, Box < dyn Any > > >,
}

pub struct Const < X > ( PhantomData<X> );

impl < F, A >
  Applied < F, A >
where
  F: 'static,
  A: 'static,
{
  pub fn get_applied(self)
    -> Box < F::Applied >
  where
    F: TypeApp < A >
  {
    self.applied.get_applied()
  }
}

pub fn get_applied < F, A >
  ( applied: Applied < F, A > )
  -> Box < F::Applied >
where
  F: 'static,
  A: 'static,
  F: TypeApp < A >,
{
  applied.applied.get_applied()
}

pub fn wrap_applied < F, A >
  ( applied: F::Applied )
  -> Applied < F, A >
where
  F: TypeApp < A >,
{
  Applied {
    applied: Box::new( applied )
  }
}
