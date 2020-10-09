use super::traits::*;
use super::structs::*;
use crate::functional::type_app::*;

pub fn extract < R, T >
  ( row: R )
  -> T
where
  R: ExtractRow < T >
{
  row.extract()
}

pub fn get_row < Row, F >
  (row: AppliedSum < Row, F > )
  -> Row::Applied
where
  F: TyCon,
  Row: RowApp < F >,
{
  row.get_row()
}

pub fn absurd < F, A >
  ( row1: AppliedSum < (), F > )
  -> A
where
  F: TyCon,
{
  match row1.get_row() {}
}

pub fn lift_sum_inject
  < Lift, Row, TargetF >
  ( lift: Lift,
    row: AppliedSum < Row, Lift::SourceF >
  ) ->
    AppliedSum < Row, Lift::InjectF >
where
  TargetF: TyCon,
  Row: SumFunctorInject,
  Lift:
    InjectLift <
      AppliedSum < Row, TargetF >,
      TargetF = TargetF
    >
{
  Row::lift_sum_inject(
    lift,
    | x | { x },
    row
  )
}