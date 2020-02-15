use crate::base::nat::*;
use async_std::sync::{ Sender, Receiver };

pub trait TyApp < A > {
  type Type;
}

pub struct Fix < F >
where
  F : TyApp < Fix < F > >
{
  unfix : Box < F :: Type >
}

pub fn fix < F >
  (x : F :: Type)
  -> Fix < F >
where
  F : TyApp < Fix < F > >
{
  Fix {
    unfix : Box::new ( x )
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Type
where
  F : TyApp < Fix < F > >
{
  *x.unfix
}

impl < A, F >
  TyApp < A > for
  Fix < F >
where
  F : TyApp < A >,
  F : TyApp < Fix < F > >,
  < F as
    TyApp < A >
  > :: Type :
    TyApp <
      Fix <
        < F as
          TyApp < A >
        > :: Type
      >
    >,
{
  type Type =
    Fix <
      < F as
        TyApp < A >
      > :: Type
    >;
}

impl < A >
  TyApp < A > for
  Z
{
  type Type = A;
}

impl < A >
  TyApp < A > for
  ()
{
  type Type = ();
}

impl < A, N >
  TyApp < A > for
  S < N >
where
  N : Nat
{
  type Type = N;
}

impl < A, X >
  TyApp < A > for
  Box < X >
where
  X : TyApp < A >
{
  type Type = Box < X :: Type >;
}

impl < A, X >
  TyApp < A > for
  Receiver < X >
where
  X : TyApp < A >
{
  type Type = Receiver < X :: Type >;
}

impl < A, X >
  TyApp < A > for
  Sender < X >
where
  X : TyApp < A >
{
  type Type = Sender < X :: Type >;
}

impl < A, X, Y >
  TyApp < A > for
  ( X, Y )
where
  X : TyApp < A >,
  Y : TyApp < A >,
{
  type Type =
    ( X :: Type,
      Y :: Type
    );
}
