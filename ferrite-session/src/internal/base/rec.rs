use super::protocol::Protocol;
use crate::internal::functional::nat::{
  S,
  Z,
};

pub trait RecApp<A>: Sized + 'static
{
  type Applied: Send + 'static;
}

pub trait HasRecApp<F, A>: Send + 'static
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F: RecApp<A>;
}

pub trait Fixed
{
  type Unfixed;

  fn fix(unfixed: Self::Unfixed) -> Self;
  fn unfix(self) -> Self::Unfixed;
}

pub trait SharedRecApp<X>
{
  type Applied;
}

pub enum Release {}

impl Protocol for Release {}

impl<T, F, A> HasRecApp<F, A> for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  F: RecApp<A, Applied = T>,
{
  fn get_applied(self: Box<T>) -> Box<T>
  {
    self
  }
}

pub struct RecX<C, F>
{
  unfix: Box<dyn HasRecApp<F, (RecX<C, F>, C)>>,
}

pub type Rec<F> = RecX<(), F>;

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

impl<C, F> Protocol for RecX<C, F>
where
  C: Send + 'static,
  F: Send + 'static,
{
}

impl<C, F> RecApp<C> for RecX<(), F>
where
  C: Send + 'static,
  F: RecApp<(RecX<C, F>, C)>,
{
  type Applied = RecX<C, F>;
}

impl<C, A> RecApp<(A, C)> for Z
where
  A: Send + 'static,
  C: Send + 'static,
{
  type Applied = A;
}

impl<N> RecApp<()> for S<N>
where
  N: Send + 'static,
{
  type Applied = S<N>;
}

impl<C, A, N> RecApp<(A, C)> for S<N>
where
  N: Send + 'static,
  C: Send + 'static,
  A: Send + 'static,
  N: RecApp<C>,
{
  type Applied = N::Applied;
}

impl<A> RecApp<A> for ()
{
  type Applied = ();
}

impl<A, X, Y> RecApp<A> for (X, Y)
where
  X: RecApp<A>,
  Y: RecApp<A>,
{
  type Applied = (X::Applied, Y::Applied);
}

impl<X> SharedRecApp<X> for Release
{
  type Applied = X;
}

impl<R> SharedRecApp<R> for ()
{
  type Applied = ();
}

impl<P, Q, R> SharedRecApp<R> for (P, Q)
where
  P: SharedRecApp<R>,
  Q: SharedRecApp<R>,
{
  type Applied = (P::Applied, Q::Applied);
}

impl<X, F> SharedRecApp<X> for RecX<(), F>
where
  F: SharedRecApp<X>,
{
  type Applied = RecX<(), F::Applied>;
}
