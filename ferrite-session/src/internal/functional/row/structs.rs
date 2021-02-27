use std::marker::PhantomData;

use serde::{
  Deserialize,
  Serialize,
};

use super::traits::*;
use crate::internal::functional::type_app::*;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Bottom {}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Sum<A, B>
{
  Inl(A),
  Inr(B),
}

pub struct AppSum<Row, F>
{
  pub row : Box<dyn HasSumApp<Row, F>>,
}

pub struct ChoiceSelector<N>
{
  phantom : PhantomData<N>,
}

pub struct Merge<T1, T2>(PhantomData<(T1, T2)>);

pub struct ElimConst {}

impl<Row, F> AppSum<Row, F>
where
  F : TyCon,
  Row : SumApp<F>,
{
  pub fn get_sum(self) -> Row::Applied
  {
    *self.row.get_sum()
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

pub fn wrap_sum_app<Row, F>(row : Row::Applied) -> AppSum<Row, F>
where
  F : TyCon,
  Row : SumApp<F>,
{
  AppSum {
    row : Box::new(row),
  }
}
