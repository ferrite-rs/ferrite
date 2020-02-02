
use std::marker::PhantomData;

pub trait Nat
  : Send + 'static
{}

#[derive(Copy, Clone)]
pub struct Z {}

#[derive(Copy, Clone)]
pub struct S < N > {
  n : PhantomData < N >
}

impl Nat for Z {}

impl < N > Nat
  for S < N >
where
  N : Nat
{}

pub fn mk_zero () -> Z {
  Z {}
}

pub fn mk_succ < N > () ->
  S < N >
where
  N : Nat
{
  S {
    n : PhantomData
  }
}