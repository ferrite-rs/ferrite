
use std::marker::PhantomData;

pub trait Nat
  : Send + 'static
{}

pub struct Z {}

pub struct Succ < N > {
  n : PhantomData < N >
}

impl Nat for Z {}

impl < N > Nat
  for Succ < N >
where
  N : Nat
{}

pub fn mk_zero () -> Z {
  Z {}
}

pub fn mk_succ < N > () ->
  Succ < N >
where
  N : Nat
{
  Succ {
    n : PhantomData
  }
}