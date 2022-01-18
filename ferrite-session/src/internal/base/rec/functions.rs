use super::{
  traits::RecApp,
  types::RecX,
};

pub fn fix<C, F>(x: F::Applied) -> RecX<C, F>
where
  C: Send + 'static,
  F: Send + 'static,
  F: RecApp<(RecX<C, F>, C)>,
{
  RecX { unfix: Box::new(x) }
}

pub fn unfix<C, F>(x: RecX<C, F>) -> F::Applied
where
  C: Send + 'static,
  F: Send + 'static,
  F: RecApp<(RecX<C, F>, C)>,
{
  *x.unfix.get_applied()
}
