use core::marker::PhantomData;

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

pub struct AppSum<'a, Row, F>
{
  pub row: Box<dyn HasSumApp<'a, Row, F> + 'a>,
}

pub struct ChoiceSelector<N>
{
  phantom: PhantomData<N>,
}

pub struct Merge<T1, T2>(PhantomData<(T1, T2)>);

pub struct ElimConst {}

impl<'a, Row, F> AppSum<'a, Row, F>
where
  F: TyCon,
  Row: SumApp<'a, F>,
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
      phantom: PhantomData,
    }
  }
}

pub fn wrap_sum_app<'a, Row, F>(row: Row::Applied) -> AppSum<'a, Row, F>
where
  F: TyCon,
  Row: SumApp<'a, F>,
  Row::Applied: Send,
{
  AppSum { row: Box::new(row) }
}
