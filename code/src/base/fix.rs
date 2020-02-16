use std::marker::PhantomData;
use crate::base::nat::*;
use async_std::sync::{ Sender, Receiver };

pub trait TyApp < A > {
  type Type;
}

pub struct Recur < A > ( PhantomData<A> );

pub struct Fix < F >
where
  F :
    TyApp <
      Recur <
        Fix < F >
      >
    >
{
  unfix : Box < F :: Type >
}

pub fn fix < F >
  (x : F :: Type)
  -> Fix < F >
where
  F :
    TyApp <
      Recur <
        Fix < F >
      >
    >
{
  Fix {
    unfix : Box::new ( x )
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Type
where
  F :
    TyApp <
      Recur <
        Fix < F >
      >
    >
{
  *x.unfix
}

impl < A, F >
  TyApp <
    A
  > for
  Fix < F >
where
  F :
    TyApp <
      S < A >
    >,
  F :
    TyApp < Recur <
      Fix < F >
    > >,
  < F as
    TyApp <
      S < A >
    >
  > :: Type :
    TyApp < Recur <
      Fix <
        < F as
          TyApp <
            S < A >
          >
        > :: Type
      >
    > >,
{
  type Type =
    Fix <
      < F as
        TyApp <
          S < A >
        >
      > :: Type
    >;
}

impl < A >
  TyApp < Recur < A > > for
  Z
{
  type Type = A;
}

impl < A >
  TyApp < S < A > > for
  Z
{
  type Type = Z;
}

impl < A, N >
  TyApp < S < A > > for
  S < N >
where
  N : TyApp < A >
{
  type Type = S < N::Type >;
}

impl < A, N >
  TyApp < Recur < A > > for
  S < N >
{
  type Type = N;
}

impl < A >
  TyApp < A > for
  ()
{
  type Type = ();
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
