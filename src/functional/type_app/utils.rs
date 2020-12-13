use std::any::Any;

use super::traits::*;
use super::structs::*;

pub fn with_applied < F, A, K >
  ( applied: Applied < F, A >,
    cont1: Box < dyn TypeAppWitnessCont < F, A, K > >
  ) -> Box < K >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  struct TypeAppWitnessContWrapper < F, A, K >
  {
    applied: Applied < F, A >,
    cont: Box < dyn TypeAppWitnessCont < F, A, K > >,
  }

  impl < F, A, K >
    TypeAppCont < F, A, Box < dyn Any > >
    for TypeAppWitnessContWrapper < F, A, K >
  where
    F: 'static,
    A: 'static,
    K: 'static,
  {
    fn on_type_app
      ( self: Box < Self > )
      -> Box < dyn Any >
    where
      F: TypeApp < A >
    {
      let res = self.cont.on_witness(
        self.applied.get_applied()
      );
      Box::new(res)
    }
  }

  let witness = applied.witness.clone_witness();

  let cont2 = TypeAppWitnessContWrapper {
    applied: applied,
    cont: cont1,
  };

  let res = witness.with_applied(Box::new(cont2));

  res.downcast().unwrap()
}
