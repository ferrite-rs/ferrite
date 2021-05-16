use std::marker::PhantomData;

use crate::internal::functional::{
  nat::*,
  row::*,
};

use super::extract::ExtractChoice;

pub struct Either<A, B>
{
  phantom: PhantomData<(A, B)>,
}

impl<A, B> ToRow for Either<A, B>
{
  type Row = (A, (B, ()));
}

pub type EitherRow<A, B> = Sum<A, Sum<B, Bottom>>;

#[allow(non_upper_case_globals)]
pub const LeftLabel: ChoiceSelector<Z> = <ChoiceSelector<Z>>::new();

#[allow(non_upper_case_globals)]
pub const RightLabel: ChoiceSelector<S<Z>> = <ChoiceSelector<S<Z>>>::new();

pub enum EitherChoice<A, B>
{
  Left(A),
  Right(B),
}

pub use EitherChoice::{
  Left,
  Right,
};

impl<A, B> ExtractChoice<Sum<A, Sum<B, Bottom>>> for EitherChoice<A, B>
{
  fn extract(row: Sum<A, Sum<B, Bottom>>) -> EitherChoice<A, B>
  {
    match row {
      Sum::Inl(a) => Left(a),
      Sum::Inr(Sum::Inl(b)) => Right(b),
      Sum::Inr(Sum::Inr(bot)) => match bot {},
    }
  }
}
