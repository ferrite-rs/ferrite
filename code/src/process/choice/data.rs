use crate::base::fix::*;

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
  TyCon < A > for
  Either < X, Y >
where
  X : TyCon < A >,
  Y : TyCon < A >,
{
  type Type =
    Either <
      X :: Type,
      Y :: Type
    >;
}