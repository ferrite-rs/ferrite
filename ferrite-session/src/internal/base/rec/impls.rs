use super::{
  traits::{
    RecApp,
    SharedRecApp,
  },
  types::{
    RecRow,
    RecX,
    Release,
    SharedRecRow,
  },
};
use crate::internal::functional::{
  nat::{
    S,
    Z,
  },
  row::*,
};

impl<C, F> RecApp<C> for RecX<(), F>
where
  C: Send,
  F: RecApp<(RecX<C, F>, C)>,
{
  type Applied = RecX<C, F>;
}

impl<C, A> RecApp<(A, C)> for Z
where
  A: Send,
  C: Send,
{
  type Applied = A;
}

impl<C, A, N> RecApp<(A, C)> for S<N>
where
  N: Send,
  C: Send,
  A: Send,
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

impl<R, Row1, Row2, Row3> ToRow for RecRow<R, Row1>
where
  R: Send,
  Row1: ToRow<Row = Row2>,
  Row2: RecApp<R, Applied = Row3>,
  Row3: RowCon,
{
  type Row = Row3;
}

impl<R, Row1, Row2, Row3> ToRow for SharedRecRow<R, Row1>
where
  R: Send,
  Row1: ToRow<Row = Row2>,
  Row2: SharedRecApp<R, Applied = Row3>,
  Row3: RowCon,
{
  type Row = Row3;
}
