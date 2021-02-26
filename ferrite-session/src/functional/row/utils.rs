use std::convert::From;

use super::{structs::*, traits::*};
use crate::functional::{base::*, type_app::*};

pub fn extract<R, T>(row : R) -> T
where
  T : From<R>,
{

  T::from(row)
}

pub fn get_row<Row, F>(row : AppliedSum<Row, F>) -> Row::Applied
where
  F : TyCon,
  Row : RowApp<F>,
{

  row.get_row()
}

pub fn get_row_borrow<Row, F>(row : &AppliedSum<Row, F>) -> &Row::Applied
where
  F : TyCon,
  Row : RowApp<F>,
{

  row.row.as_ref().get_row_borrow()
}

pub fn absurd<F, A>(row1 : AppliedSum<(), F>) -> A
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
  sum : AppliedSum<Row, F1>,
) -> AppliedSum<Row, F2>
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
  row : AppliedSum<Row, Lift::SourceF>,
) -> AppliedSum<Row, Lift::InjectF>
where
  TargetF : TyCon,
  Row : SumFunctorInject,
  Lift : InjectLift<AppliedSum<Row, TargetF>, TargetF = TargetF>,
{

  Row::lift_sum_inject(lift, |x| x, row)
}
