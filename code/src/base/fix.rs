use crate::base::nat::*;
use async_std::sync::{ Sender, Receiver };

pub trait TyCon < A > {
  type Type;
}

pub struct Fix < F >
where
  F : TyCon < Fix < F > >
{
  unfix : Box < F :: Type >
}

pub fn fix < F >
  (x : F :: Type)
  -> Fix < F >
where
  F : TyCon < Fix < F > >
{
  Fix {
    unfix : Box::new ( x )
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Type
where
  F : TyCon < Fix < F > >
{
  *x.unfix
}

impl < A, F >
  TyCon < A > for
  Fix < F >
where
  F : TyCon < A >,
  F : TyCon < Fix < F > >,
  < F as
    TyCon < A >
  > :: Type :
    TyCon <
      Fix <
        < F as
          TyCon < A >
        > :: Type
      >
    >,
{
  type Type =
    Fix <
      < F as
        TyCon < A >
      > :: Type
    >;
}

impl < A >
  TyCon < A > for
  Z
{
  type Type = A;
}

impl < A >
  TyCon < A > for
  ()
{
  type Type = ();
}

impl < A, N >
  TyCon < A > for
  Succ < N >
where
  N : Nat
{
  type Type = N;
}

impl < A, X >
  TyCon < A > for
  Box < X >
where
  X : TyCon < A >
{
  type Type = Box < X :: Type >;
}

impl < A, X >
  TyCon < A > for
  Receiver < X >
where
  X : TyCon < A >
{
  type Type = Receiver < X :: Type >;
}

impl < A, X >
  TyCon < A > for
  Sender < X >
where
  X : TyCon < A >
{
  type Type = Sender < X :: Type >;
}

impl < A, X, Y >
  TyCon < A > for
  ( X, Y )
where
  X : TyCon < A >,
  Y : TyCon < A >,
{
  type Type =
    ( X :: Type,
      Y :: Type
    );
}
