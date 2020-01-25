
use std::marker::PhantomData;

pub trait Nat
  : Send + 'static
{}

pub struct Zero {}

pub struct Succ < N > {
  n : PhantomData < N >
}

impl Nat for Zero {}

impl < N > Nat
  for Succ < N >
where
  N : Nat
{}

pub fn mk_zero () -> Zero {
  Zero {}
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