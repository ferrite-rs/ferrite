use std::marker::PhantomData;
use crate::base::nat::*;
use async_std::sync::{ Sender, Receiver };

/*
  class TyApp self where
    type family Apply self a
 */
pub trait TyApp < A > {
  type Applied;
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
  unfix : Box < F :: Applied >
}

pub fn fix < F >
  (x : F :: Applied)
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
  -> F :: Applied
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
  > :: Applied :
    TyApp < Recur <
      Fix <
        < F as
          TyApp <
            S < A >
          >
        > :: Applied
      >
    > >,
{
  type Applied =
    Fix <
      < F as
        TyApp <
          S < A >
        >
      > :: Applied
    >;
}

impl < A >
  TyApp < Recur < A > > for
  Z
{
  type Applied = A;
}

impl
  TyApp < Z > for
  Z
{
  type Applied = Z;
}

impl < A >
  TyApp < S < A > > for
  Z
{
  type Applied = Z;
}

impl < A, N >
  TyApp < S < A > > for
  S < N >
where
  N : TyApp < A >
{
  type Applied = S < N::Applied >;
}

impl < A, N >
  TyApp < Recur < A > > for
  S < N >
{
  type Applied = N;
}

impl < N >
  TyApp < Z > for
  S < N >
{
  type Applied = S < N >;
}

impl < A >
  TyApp < A > for
  ()
{
  type Applied = ();
}

impl < A, X >
  TyApp < A > for
  Box < X >
where
  X : TyApp < A >
{
  type Applied = Box < X :: Applied >;
}

impl < A, X >
  TyApp < A > for
  Receiver < X >
where
  X : TyApp < A >
{
  type Applied = Receiver < X :: Applied >;
}

impl < A, X >
  TyApp < A > for
  Sender < X >
where
  X : TyApp < A >
{
  type Applied = Sender < X :: Applied >;
}

impl < A, X, Y >
  TyApp < A > for
  ( X, Y )
where
  X : TyApp < A >,
  Y : TyApp < A >,
{
  type Applied =
    ( X :: Applied,
      Y :: Applied
    );
}
