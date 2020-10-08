use std::marker::PhantomData;

use crate::functional::nat::*;

pub trait RecApp < A >
  : Sized + 'static
{
  type Applied : Send + 'static;
}

pub trait HasRecApp < F, A >
  : Send + 'static
{
  fn get_applied
    ( self: Box < Self > )
    -> Box < F::Applied >
  where
    F: RecApp < A >
  ;
}

impl < T, F, A >
  HasRecApp < F, A >
  for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  F: RecApp < A, Applied=T >
{
  fn get_applied (self: Box < T >) -> Box < T >
  { self }
}

pub struct Unfix < A > ( PhantomData<A> );

pub struct Fix < F >
{
  unfix :
    Box < dyn
      HasRecApp <
        F,
        Unfix <
          Fix < F >
        >
      >
    >
}

pub fn fix < F >
  (x : F :: Applied)
  -> Fix < F >
where
  F : Send + 'static,
  F :
    RecApp <
      Unfix <
        Fix < F >
      >
    >
{
  Fix {
    unfix: Box::new( x )
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Applied
where
  F : Send + 'static,
  F :
    RecApp <
      Unfix <
        Fix < F >
      >
    >
{
  *x.unfix.get_applied()
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
where
  A: Send + 'static,
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
where
  N: Send + 'static,
{
  type Applied = N;
}

impl < N >
  RecApp < Z > for
  S < N >
where
  N: Send + 'static,
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
