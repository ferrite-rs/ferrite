use crate::functional::{nat::*, row::*};

pub type Either<A, B> = (A, (B, ()));

pub type EitherRow<A, B> = Sum<A, Sum<B, Bottom>>;

#[allow(non_upper_case_globals)]

pub const LeftLabel : ChoiceSelector<Z> = <ChoiceSelector<Z>>::new();

#[allow(non_upper_case_globals)]

pub const RightLabel : ChoiceSelector<S<Z>> = <ChoiceSelector<S<Z>>>::new();

pub enum EitherChoice<A, B>
{
  Left(A),
  Right(B),
}

pub use EitherChoice::{Left, Right};

impl<A, B> From<Sum<A, Sum<B, Bottom>>> for EitherChoice<A, B>
{
  fn from(row : Sum<A, Sum<B, Bottom>>) -> EitherChoice<A, B>
  {

    match row {
      Sum::Inl(a) => Left(a),
      Sum::Inr(Sum::Inl(b)) => Right(b),
      Sum::Inr(Sum::Inr(bot)) => match bot {},
    }
  }
}
