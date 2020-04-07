use crate::base::{ TyApp };

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
  TyApp < A > for
  Either < X, Y >
where
  X : TyApp < A >,
  Y : TyApp < A >,
{
  type Applied =
    Either <
      X :: Applied,
      Y :: Applied
    >;
}
