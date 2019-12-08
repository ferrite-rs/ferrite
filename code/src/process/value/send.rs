use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };
use crate::fix::{ AlgebraT };

pub struct SendValue < T, P >
{
  value: PhantomData<T>,
  process: PhantomData<P>
}

impl < T, P > Process for SendValue < T, P >
where
  T : Send,
  P : Process
{
  type Value = (
    T,
    Receiver < P::Value >
  );
}

impl < T, P, R >
  AlgebraT < R > for
  SendValue < T, P >
where
  T : Send,
  P : AlgebraT < R >,
  < P as
    AlgebraT < R >
  > :: Algebra
    : Process
{
  type Algebra =
    SendValue <
      T,
      < P as
        AlgebraT < R >
      > :: Algebra
    >;
}