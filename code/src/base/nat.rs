
pub trait Nat
  : Send + 'static
{
  fn nat() -> Self;
}

#[derive(Copy, Clone)]
pub struct Z ();

#[derive(Copy, Clone)]
pub struct S < N > (N);

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
    S ( N::nat() )
  }
}

pub fn succ < N >
  ( n : N )
  -> S < N >
{
  S ( n )
}
