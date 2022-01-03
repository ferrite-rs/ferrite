use super::structs::*;
use crate::internal::functional::{
  base::*,
  type_app::*,
};

pub trait RowCon: Sized {}

pub trait ToRow
{
  type Row;
}

pub trait SumApp<'a, F>: RowCon
where
  F: TyCon,
{
  type Applied: Sized + Send + 'a;
}

// Flatten the App wrappers in SumApp
pub trait FlattenSumApp<'a, F>: SumApp<'a, F>
where
  F: TyCon,
{
  type FlattenApplied;

  fn unflatten_sum(row: Self::FlattenApplied) -> Self::Applied;

  fn flatten_sum(row: AppSum<'a, Self, F>) -> Self::FlattenApplied;
}

pub trait HasSumApp<'a, Row, F>: Send
{
  fn get_sum(self: Box<Self>) -> Box<Row::Applied>
  where
    F: TyCon,
    Row: SumApp<'a, F>;

  fn get_sum_borrow<'b>(&'b self) -> &'b Row::Applied
  where
    F: TyCon,
    Row: SumApp<'a, F>;
}

pub trait SplitRow: Sized + RowCon
{
  fn split_row<'a, F1: 'a, F2: 'a>(
    row: AppSum<'a, Self, Merge<F1, F2>>
  ) -> (AppSum<'a, Self, F1>, AppSum<'a, Self, F2>)
  where
    F1: TyCon,
    F2: TyCon,
    Self: 'a;
}

pub trait SumFunctor: RowCon
{
  fn lift_sum<'a, T: 'a, F1: 'a, F2: 'a>(
    lift: T,
    sum: AppSum<'a, Self, F1>,
  ) -> AppSum<'a, Self, F2>
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation<'a, F1, F2>,
    Self: 'a;
}

pub trait InjectLift<'a, Root>
{
  type SourceF: TyCon;

  type TargetF: TyCon;

  type InjectF: TyCon;

  fn lift_field<A>(
    self,
    inject: impl Fn(App<'a, Self::TargetF, A>) -> Root + Send + 'a,
    row: App<'a, Self::SourceF, A>,
  ) -> App<'a, Self::InjectF, A>
  where
    A: Send + 'a;
}

pub trait SumFunctorInject: RowCon
{
  fn lift_sum_inject<'a, L, Root, Inject>(
    ctx: L,
    inject: Inject,
    sum: AppSum<'a, Self, L::SourceF>,
  ) -> AppSum<'a, Self, L::InjectF>
  where
    L: InjectLift<'a, Root> + Send,
    Inject: Fn(AppSum<'a, Self, L::TargetF>) -> Root + Send + 'a,
    Root: Send,
    Self: 'a,
    L::SourceF: 'a,
    L::InjectF: 'a,
    L::TargetF: 'a;
}

pub trait IntersectSum: RowCon
{
  fn intersect_sum<'a, F1: 'a, F2: 'a>(
    row1: AppSum<'a, Self, F1>,
    row2: AppSum<'a, Self, F2>,
  ) -> Option<AppSum<'a, Self, Merge<F1, F2>>>
  where
    F1: TyCon,
    F2: TyCon,
    Self: 'a;
}

pub trait ElimField<'a, F, R>
where
  F: TyCon,
{
  fn elim_field<A: 'a>(
    self,
    a: App<'a, F, A>,
  ) -> R;
}

pub trait ElimSum: RowCon
{
  fn elim_sum<'a, F: 'a, E, R>(
    elim_field: E,
    row: AppSum<'a, Self, F>,
  ) -> R
  where
    Self: 'a,
    F: TyCon,
    E: ElimField<'a, F, R>;
}

pub trait Prism<Row>
where
  Row: RowCon,
{
  type Elem;

  fn inject_elem<'a, F: 'a + Send>(
    elem: App<'a, F, Self::Elem>
  ) -> AppSum<'a, Row, F>
  where
    F: TyCon,
    Row: 'a;

  fn extract_elem<'a, F: 'a + Send>(
    row: AppSum<'a, Row, F>
  ) -> Option<App<'a, F, Self::Elem>>
  where
    F: TyCon,
    Row: 'a;
}
