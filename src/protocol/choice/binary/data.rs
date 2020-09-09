use crate::base::{ TypeApp };

pub enum Choice {
  Left,
  Right
}

pub enum Either < S, T >
{
  Left(S),
  Right(T)
}

impl < A, X, Y >
  TypeApp < A > for
  Either < X, Y >
where
  X : TypeApp < A >,
  Y : TypeApp < A >,
{
  type Applied =
    Either <
      X :: Applied,
      Y :: Applied
    >;
}
