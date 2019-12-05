use std::marker::PhantomData;

use crate::fix::fix::{ Fix, AlgebraT };

pub enum ListF < A, R >
where
  A : Sized
{
  NilF,
  ConsF( A, Box < R > )
}

pub struct ListT < A > {
  a: PhantomData < A >
}

impl
  < A, R >
  AlgebraT < R >
  for ListT < A >
where
  A : Sized
{
  type Algebra = ListF < A, R >;
}

pub type List < A > =
  Fix <
    ListT < A >
  >;

pub fn nil_f
  < A, R >
  ()
  -> ListF < A, R >
where
  A : Sized
{
  ListF::NilF
}

pub fn cons_f
  < A, R >
  (a: A, r: R)
  -> ListF < A, R >
where
  A : Sized
{
  ListF::ConsF( a, Box::new (r) )
}
