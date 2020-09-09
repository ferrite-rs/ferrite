use std::marker::PhantomData;

pub trait Nat
  : Send + 'static
{
  const VAL : Self;

  fn nat() -> Self;
}

#[derive(Copy, Clone)]
pub struct Z ();

#[derive(Copy, Clone)]
pub struct S < N > ( pub PhantomData<N> );

impl Nat for Z {
  const VAL : Z = Z();

  fn nat() -> Z { Z() }
}

impl < N > Nat
  for S < N >
where
  N : Nat
{
  const VAL : S<N> = S( PhantomData );

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
