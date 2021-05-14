use std::marker::PhantomData;

use super::protocol::Protocol;
use crate::internal::functional::{
  base::*,
  nat::{
    S,
    Z,
  },
  row::*,
  type_app::*,
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

pub trait HasSharedRecApp<F, A>: Send + 'static
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F: SharedRecApp<A>;
}

impl<T, F, A> HasSharedRecApp<F, A> for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  F: SharedRecApp<A, Applied = T>,
{
  fn get_applied(self: Box<T>) -> Box<T>
  {
    self
  }
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

pub enum LinearMode {}
pub enum SharedMode {}

pub struct RecRow<Mode, R, Row>
{
  phantom: PhantomData<(Mode, R, Row)>,
}

pub type LinearRecRow<R, Row> = RecRow<LinearMode, R, Row>;
pub type SharedRecRow<R, Row> = RecRow<SharedMode, R, Row>;

impl<Mode, R, Row> Protocol for RecRow<Mode, R, Row>
where
  R: Send + 'static,
  Mode: Send + 'static,
  Row: Protocol,
{
}

impl<Mode, R, Row> RowCon for RecRow<Mode, R, Row>
where
  R: Send + 'static,
  Row: Send + 'static,
  Mode: Send + 'static,
{
}

pub trait RecRowApp<Row, R>
{
  type Applied;
}

impl<R, Row> RecRowApp<Row, R> for LinearMode
where
  Row: RecApp<R>,
{
  type Applied = Row::Applied;
}

impl<R, Row> RecRowApp<Row, R> for SharedMode
where
  Row: SharedRecApp<R>,
{
  type Applied = Row::Applied;
}

impl<R1, R2, Row1, Row2, Row3> RecApp<R2> for RecRow<SharedMode, R1, Row1>
where
  R1: Send + 'static,
  Row1: Send + 'static,
  Row3: Send + 'static,
  Row1: SharedRecApp<R2, Applied = Row2>,
  Row2: RecApp<R2, Applied = Row3>,
{
  type Applied = Row3;
}

impl<F, R, Row1, Row2, Mode> SumApp<F> for RecRow<Mode, R, Row1>
where
  F: TyCon,
  R: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
{
  type Applied = AppSum<Row2, F>;
}

impl<F, R, Row1, Row2, Row3, Mode> FlattenSumApp<F> for RecRow<Mode, R, Row1>
where
  F: TyCon,
  R: Send + 'static,
  Row1: Send + 'static,
  Row3: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  Row2: FlattenSumApp<F, FlattenApplied = Row3>,
{
  type FlattenApplied = Row3;

  fn unflatten_sum(row: Self::FlattenApplied) -> Self::Applied
  {
    wrap_sum_app(Row2::unflatten_sum(row))
  }

  fn flatten_sum(row1: AppSum<Self, F>) -> Self::FlattenApplied
  {
    Row2::flatten_sum(row1.get_sum())
  }
}

impl<R, Row1, Row2, Mode> SplitRow for RecRow<Mode, R, Row1>
where
  R: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  Row2: SplitRow,
{
  fn split_row<F1, F2>(
    row1: AppSum<Self, Merge<F1, F2>>
  ) -> (AppSum<Self, F1>, AppSum<Self, F2>)
  where
    F1: TyCon,
    F2: TyCon,
  {
    let (row2, row3) = Row2::split_row(row1.get_sum());
    (wrap_sum_app(row2), wrap_sum_app(row3))
  }
}

impl<R, Row1, Row2, Mode> SumFunctor for RecRow<Mode, R, Row1>
where
  R: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  Row2: SumFunctor,
{
  fn lift_sum<T, F1, F2>(
    lift: &T,
    sum: AppSum<Self, F1>,
  ) -> AppSum<Self, F2>
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation<F1, F2>,
  {
    wrap_sum_app(Row2::lift_sum(lift, sum.get_sum()))
  }
}

impl<R, Row1, Row2, Mode> SumFunctorInject for RecRow<Mode, R, Row1>
where
  R: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  Row2: SumFunctorInject,
{
  fn lift_sum_inject<L, Root, Inject>(
    ctx: L,
    inject1: Inject,
    sum: AppSum<Self, L::SourceF>,
  ) -> AppSum<Self, L::InjectF>
  where
    L: InjectLift<Root>,
    Inject: Fn(AppSum<Self, L::TargetF>) -> Root + Send + 'static,
  {
    let inject2 =
      move |row: AppSum<Row2, L::TargetF>| inject1(wrap_sum_app(row));

    wrap_sum_app(Row2::lift_sum_inject(ctx, inject2, sum.get_sum()))
  }
}

impl<R, Row1, Row2, Mode> IntersectSum for RecRow<Mode, R, Row1>
where
  R: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  Row2: IntersectSum,
{
  fn intersect_sum<F1, F2>(
    row1: AppSum<Self, F1>,
    row2: AppSum<Self, F2>,
  ) -> Option<AppSum<Self, Merge<F1, F2>>>
  where
    F1: TyCon,
    F2: TyCon,
  {
    Row2::intersect_sum(row1.get_sum(), row2.get_sum())
      .map(|row3| wrap_sum_app(row3))
  }
}

impl<R, Row1, Row2, Mode> ElimSum for RecRow<Mode, R, Row1>
where
  R: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Mode: Send + 'static,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  Row2: ElimSum,
{
  fn elim_sum<F, E, Res>(
    elim_field: E,
    row: AppSum<Self, F>,
  ) -> Res
  where
    F: TyCon,
    E: ElimField<F, Res>,
  {
    Row2::elim_sum(elim_field, row.get_sum())
  }
}

impl<N, R, Row1, Row2, Mode> Prism<RecRow<Mode, R, Row1>> for N
where
  R: Send + 'static,
  Row1: Send + 'static,
  Mode: Send + 'static,
  Row2: RowCon,
  Mode: RecRowApp<Row1, R, Applied = Row2>,
  N: Prism<Row2>,
{
  type Elem = N::Elem;

  fn inject_elem<F>(
    elem: App<F, Self::Elem>
  ) -> AppSum<RecRow<Mode, R, Row1>, F>
  where
    F: TyCon,
  {
    wrap_sum_app(N::inject_elem(elem))
  }

  fn extract_elem<F>(
    row: AppSum<RecRow<Mode, R, Row1>, F>
  ) -> Option<App<F, Self::Elem>>
  where
    F: TyCon,
  {
    N::extract_elem(row.get_sum())
  }
}
