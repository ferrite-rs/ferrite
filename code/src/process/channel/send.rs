use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };
use crate::fix::{ AlgebraT };

pub struct SendChannel < P, Q >  {
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl < P, Q >
  Process for
  SendChannel < P, Q >
where
  P: Process,
  Q: Process
{
  type Value = (
    Receiver< P::Value >,
    Receiver< Q::Value >
  );
}

impl < P, Q, R >
  AlgebraT < R > for
  SendChannel < P, Q >
where
  P : AlgebraT < R >,
  Q : AlgebraT < R >,
  < P as
    AlgebraT < R >
  > :: Algebra
    : Process,
  < Q as
    AlgebraT < R >
  > :: Algebra
    : Process
{
  type Algebra =
    SendChannel <
      < P as
        AlgebraT < R >
      > :: Algebra,
      < Q as
        AlgebraT < R >
      > :: Algebra
    >;
}