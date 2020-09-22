use std::mem::transmute;
use std::marker::PhantomData;

use crate::base::nat::*;

pub trait RecApp < A > {
  type Applied;
}

pub struct Unfix < A > ( PhantomData<A> );

pub struct Fix < F >
{
  unfix : Box < F >
}

pub fn fix < F >
  (x : F :: Applied)
  -> Fix < F >
where
  F :
    RecApp <
      Unfix <
        Fix < F >
      >
    >
{
  unsafe {
    let wrapped : Box < F > =
      transmute ( Box::new ( x ) );

    Fix {
      unfix : wrapped
    }
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Applied
where
  F :
    RecApp <
      Unfix <
        Fix < F >
      >
    >
{
  unsafe {
    let wrapped : Box < F::Applied > =
      transmute ( x.unfix );

    *wrapped
  }
}

impl < A, F >
  RecApp < A >
  for Fix < F >
where
  F :
    RecApp <
      S < A >
    >,
  F :
    RecApp < Unfix <
      Fix < F >
    > >,
  < F as
    RecApp <
      S < A >
    >
  > :: Applied :
    RecApp < Unfix <
      Fix <
        < F as
          RecApp <
            S < A >
          >
        > :: Applied
      >
    > >,
{
  type Applied =
    Fix <
      < F as
        RecApp <
          S < A >
        >
      > :: Applied
    >;
}

impl < A >
  RecApp < Unfix < A > > for
  Z
{
  type Applied = A;
}

impl
  RecApp < Z > for
  Z
{
  type Applied = Z;
}

impl < A >
  RecApp < S < A > > for
  Z
{
  type Applied = Z;
}

impl < A, N >
  RecApp < S < A > > for
  S < N >
where
  N : RecApp < A >
{
  type Applied = S < N::Applied >;
}

impl < A, N >
  RecApp < Unfix < A > > for
  S < N >
{
  type Applied = N;
}

impl < N >
  RecApp < Z > for
  S < N >
{
  type Applied = S < N >;
}

impl < A >
  RecApp < A > for
  ()
{
  type Applied = ();
}

impl < A, X, Y >
  RecApp < A > for
  ( X, Y )
where
  X : RecApp < A >,
  Y : RecApp < A >,
{
  type Applied =
    ( X :: Applied,
      Y :: Applied
    );
}
