use std::{
  any::Any,
  marker::PhantomData,
};

use serde::{
  Deserialize,
  Serialize,
};

use super::traits::*;
use crate::functional::type_app::*;

#[derive(Copy, Clone, Serialize, Deserialize)]

pub enum Bottom {}

#[derive(Copy, Clone, Serialize, Deserialize)]

pub enum Sum<A, B>
{
  Inl(A),
  Inr(B),
}

pub struct AppliedSum<Row, F>
{
  pub row : Box<dyn HasRowWitness<Row, F, Box<dyn Any>>>,
}

pub struct ChoiceSelector<N>
{
  phantom : PhantomData<N>,
}

pub struct Merge<T1, T2>(PhantomData<(T1, T2)>);

pub struct ElimConst {}

impl<Row, F> AppliedSum<Row, F>
where
  F : TyCon,
  Row : RowApp<F>,
{
  pub fn get_row(self) -> Row::Applied
  {
    *self.row.get_row()
  }
}

impl<N> ChoiceSelector<N>
{
  pub const fn new() -> ChoiceSelector<N>
  {
    ChoiceSelector {
      phantom : PhantomData,
    }
  }
}

pub fn cloak_row<Row, F>(row : Row::Applied) -> AppliedSum<Row, F>
where
  F : TyCon,
  Row : RowApp<F>,
{
  AppliedSum {
    row : Box::new(row),
  }
}
