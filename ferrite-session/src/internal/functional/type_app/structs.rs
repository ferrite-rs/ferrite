use std::marker::PhantomData;

use super::traits::*;

pub struct App<F, A>
{
  pub applied: Box<dyn HasTypeApp<F, A>>,
}

pub struct Const<X>(PhantomData<X>);

impl<F, A> App<F, A>
where
  F: 'static,
  A: 'static,
{
  pub fn get_applied(self) -> F::Applied
  where
    F: TypeApp<A>,
  {
    *self.applied.get_applied()
  }
}

pub fn get_applied<F, A>(applied: App<F, A>) -> F::Applied
where
  F: 'static,
  A: 'static,
  F: TypeApp<A>,
{
  *applied.applied.get_applied()
}

pub fn wrap_type_app<F, A>(applied: F::Applied) -> App<F, A>
where
  F: TypeApp<A>,
{
  App {
    applied: Box::new(applied),
  }
}
