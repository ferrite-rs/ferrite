use std::marker::PhantomData;

use super::traits::*;

pub struct App<'a, F, A>
{
  pub applied: Box<dyn HasTypeApp<'a, F, A> + 'a>,
}

pub struct Const<X>(PhantomData<X>);

impl<'a, F, A> App<'a, F, A>
{
  pub fn get_applied(self) -> F::Applied
  where
    F: TypeApp<'a, A>,
  {
    *self.applied.get_applied()
  }
}

pub fn get_applied<'a, F, A>(applied: App<'a, F, A>) -> F::Applied
where
  F: TypeApp<'a, A>,
{
  *applied.applied.get_applied()
}

pub fn wrap_type_app<'a, F, A>(applied: F::Applied) -> App<'a, F, A>
where
  F: TypeApp<'a, A>,
{
  App {
    applied: Box::new(applied),
  }
}
