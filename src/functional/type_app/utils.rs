use std::any::Any;

use super::traits::*;
use super::structs::*;

struct TypeAppWitnessContWrapper < F, A, K >
{
  cont: Box < dyn TypeAppWitnessCont < F, A, K > >,
}

impl < F, A, K >
  TypeAppWitnessCont < F, A, Box < dyn Any > >
  for TypeAppWitnessContWrapper < F, A, K >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  fn on_witness
    ( self: Box < Self >,
      applied: Box < F::Applied >,
    ) -> Box < dyn Any >
  where
    F: TypeApp < A >
  {
    let res = self.cont.on_witness(applied);
    Box::new(res)
  }
}

pub fn with_applied < F, A, K >
  ( applied: Applied < F, A >,
    cont1: Box < dyn TypeAppWitnessCont < F, A, K > >
  ) -> Box < K >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  let cont2 = TypeAppWitnessContWrapper {
    cont: cont1,
  };

  let res = applied.applied.with_applied(Box::new(cont2));
  res.downcast().unwrap()
}
