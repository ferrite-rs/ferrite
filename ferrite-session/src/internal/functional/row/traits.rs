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
