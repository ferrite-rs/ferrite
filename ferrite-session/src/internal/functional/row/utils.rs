use std::convert::From;

use super::{
  structs::*,
  traits::*,
};
use crate::internal::functional::{
  base::*,
  type_app::*,
};

pub fn extract<R, T>(row : R) -> T
where
  T : From<R>,
{
  T::from(row)
}

pub fn get_row<Row, F>(row : AppSum<Row, F>) -> Row::Applied
where
  F : TyCon,
  Row : SumApp<F>,
{
  row.get_row()
}

pub fn get_row_borrow<Row, F>(row : &AppSum<Row, F>) -> &Row::Applied
where
  F : TyCon,
  Row : SumApp<F>,
{
  row.row.as_ref().get_row_borrow()
}

pub fn absurd<F, A>(row1 : AppSum<(), F>) -> A
where
  F : TyCon,
{
  match row1.get_row() {}
}

// lift_sum : forall row f g
//   . (forall x . f x -> g x)
//   -> row f
//   -> row g
pub fn lift_sum<Row, F1, F2, Lift>(
  lift : &Lift,
  sum : AppSum<Row, F1>,
) -> AppSum<Row, F2>
where
  F1 : TyCon,
  F2 : TyCon,
  Row : SumFunctor,
  Lift : NaturalTransformation<F1, F2>,
{
  Row::lift_sum(lift, sum)
}

pub fn lift_sum_inject<Lift, Row, TargetF>(
  lift : Lift,
  row : AppSum<Row, Lift::SourceF>,
) -> AppSum<Row, Lift::InjectF>
where
  TargetF : TyCon,
  Row : SumFunctorInject,
  Lift : InjectLift<AppSum<Row, TargetF>, TargetF = TargetF>,
{
  Row::lift_sum_inject(lift, |x| x, row)
}
