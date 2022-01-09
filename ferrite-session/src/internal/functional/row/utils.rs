use core::convert::Into;

use super::{
  structs::*,
  traits::*,
};
use crate::internal::functional::{
  base::*,
  type_app::*,
};

pub fn extract<'a, Row, F, T1, T2>(row: AppSum<'a, Row, F>) -> T2
where
  F: TyCon,
  Row: FlattenSumApp<'a, F, FlattenApplied = T1>,
  T1: Into<T2>,
{
  Row::flatten_sum(row).into()
}

pub fn get_sum<'a, Row, F>(row: AppSum<'a, Row, F>) -> Row::Applied
where
  F: TyCon,
  Row: SumApp<'a, F>,
{
  row.get_sum()
}

pub fn get_sum_borrow<'a, 'b, Row, F>(
  row: &'b AppSum<'a, Row, F>
) -> &'b Row::Applied
where
  F: TyCon,
  Row: SumApp<'a, F>,
{
  row.row.as_ref().get_sum_borrow()
}

#[allow(unreachable_code)]
pub fn absurd<F, A>(row1: AppSum<(), F>) -> A
where
  F: TyCon,
{
  match row1.get_sum() {}
}

// lift_sum : forall row f g
//   . (forall x . f x -> g x)
//   -> row f
//   -> row g
pub fn lift_sum<'a, Row: 'a, F1: 'a, F2: 'a, Lift: 'a>(
  lift: Lift,
  sum: AppSum<'a, Row, F1>,
) -> AppSum<'a, Row, F2>
where
  F1: TyCon,
  F2: TyCon,
  Row: SumFunctor,
  Lift: NaturalTransformation<'a, F1, F2>,
{
  Row::lift_sum(lift, sum)
}

pub fn lift_sum_inject<'a, Lift: 'a, Row: 'a, TargetF: 'a>(
  lift: Lift,
  row: AppSum<'a, Row, Lift::SourceF>,
) -> AppSum<'a, Row, Lift::InjectF>
where
  TargetF: TyCon,
  Row: SumFunctorInject,
  Lift: InjectLift<'a, AppSum<'a, Row, TargetF>, TargetF = TargetF> + Send,
  Lift::SourceF: 'a,
  Lift::InjectF: 'a,
  Lift::TargetF: 'a,
{
  Row::lift_sum_inject(lift, |x| x, row)
}
