use std::marker::PhantomData;

pub trait Nat
  : Send + 'static
{
  fn nat() -> Self;
}

#[derive(Copy, Clone)]
pub struct Z ();

#[derive(Copy, Clone)]
pub struct S < N > (PhantomData<N>);

impl Nat for Z {
  fn nat() -> Z { Z() }
}

impl < N > Nat
  for S < N >
where
  N : Nat
{
  fn nat() ->
    S < N >
  {
    S ( PhantomData )
  }
}

pub fn succ < N >
  ( _ : N )
  -> S < N >
{
  S ( PhantomData )
}
