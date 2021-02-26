use std::marker::PhantomData;

use super::protocol::Protocol;
use crate::functional::nat::{
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
    F : RecApp<A>;
}

pub trait SharedRecApp<X>
{
  type Applied;
}

impl<T, F, A> HasRecApp<F, A> for T
where
  F : 'static,
  A : 'static,
  T : Send + 'static,
  F : RecApp<A, Applied = T>,
{
  fn get_applied(self: Box<T>) -> Box<T>
  {
    self
  }
}

pub struct Unfix<A>(PhantomData<A>);

pub struct Rec<F>
{
  unfix : Box<dyn HasRecApp<F, Unfix<Rec<F>>>>,
}

pub fn fix<F>(x : F::Applied) -> Rec<F>
where
  F : Send + 'static,
  F : RecApp<Unfix<Rec<F>>>,
{
  Rec {
    unfix : Box::new(x),
  }
}

pub fn unfix<F>(x : Rec<F>) -> F::Applied
where
  F : Send + 'static,
  F : RecApp<Unfix<Rec<F>>>,
{
  *x.unfix.get_applied()
}

impl<F> Protocol for Rec<F> where F : Send + 'static {}

impl<F> Protocol for Unfix<F> where F : Send + 'static {}

impl<A, F> RecApp<A> for Rec<F>
where
  F : RecApp<S<A>>,
  F : RecApp<Unfix<Rec<F>>>,
  <F as RecApp<S<A>>>::Applied :
    RecApp<Unfix<Rec<<F as RecApp<S<A>>>::Applied>>>,
{
  type Applied = Rec<<F as RecApp<S<A>>>::Applied>;
}

impl<A> RecApp<Unfix<A>> for Z
where
  A : Send + 'static,
{
  type Applied = A;
}

impl RecApp<Z> for Z
{
  type Applied = Z;
}

impl<A> RecApp<S<A>> for Z
{
  type Applied = Z;
}

impl<A, N> RecApp<S<A>> for S<N>
where
  N : RecApp<A>,
{
  type Applied = S<N::Applied>;
}

impl<A, N> RecApp<Unfix<A>> for S<N>
where
  N : Send + 'static,
{
  type Applied = N;
}

impl<N> RecApp<Z> for S<N>
where
  N : Send + 'static,
{
  type Applied = S<N>;
}

impl<A> RecApp<A> for ()
{
  type Applied = ();
}

impl<A, X, Y> RecApp<A> for (X, Y)
where
  X : RecApp<A>,
  Y : RecApp<A>,
{
  type Applied = (X::Applied, Y::Applied);
}

impl<X> SharedRecApp<X> for Z
{
  type Applied = X;
}

impl<R> SharedRecApp<R> for ()
{
  type Applied = ();
}

impl<P, Q, R> SharedRecApp<R> for (P, Q)
where
  P : SharedRecApp<R>,
  Q : SharedRecApp<R>,
{
  type Applied = (P::Applied, Q::Applied);
}
