use std::marker::PhantomData;

pub trait SealedNat {}

pub trait Nat: SealedNat + Send + Copy + 'static
{
  #[allow(non_upper_case_globals)]
  const Value: Self;

  fn nat() -> Self;
}

#[derive(Copy, Clone)]
pub struct Z;

#[derive(Copy, Clone)]
pub struct S<N>(pub PhantomData<N>);

impl SealedNat for Z {}

impl Nat for Z
{
  #[allow(non_upper_case_globals)]
  const Value: Z = Z;

  fn nat() -> Z
  {
    Z
  }
}

impl<N> SealedNat for S<N> where N: Nat {}

impl<N> Nat for S<N>
where
  N: Nat,
{
  #[allow(non_upper_case_globals)]
  const Value: S<N> = S(PhantomData);

  fn nat() -> S<N>
  {
    S(PhantomData)
  }
}

pub fn succ<N>(_: N) -> S<N>
{
  S(PhantomData)
}
