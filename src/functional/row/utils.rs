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
  -> Box < Row::Applied >
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
  let row2 = *row1.get_row();
  match row2 {}
}
