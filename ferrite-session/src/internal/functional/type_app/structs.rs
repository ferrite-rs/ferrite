use super::traits::*;

pub struct App<'a, F, A>
{
  pub applied: Box<dyn HasTypeApp<'a, F, A> + 'a>,
}

impl<'a, F, A> App<'a, F, A>
{
  pub fn new(applied: F::Applied) -> App<'a, F, A>
  where
    F: TypeApp<'a, A>,
    F::Applied: 'a,
  {
    App {
      applied: Box::new(applied),
    }
  }

  pub fn get_applied(self) -> F::Applied
  where
    F: TypeApp<'a, A>,
  {
    *self.applied.get_applied()
  }
}
