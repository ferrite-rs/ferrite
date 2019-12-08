use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };
use crate::process::choice::data::{ Either };
use crate::fix::{ AlgebraT };

/*
  data InternalChoice p q = InternalChoice
 */
pub struct InternalChoice < P, Q >
{
  p : PhantomData < P >,
  q : PhantomData < Q >
}

impl
  < P, Q >
  Process for
  InternalChoice < P, Q >
where
  P: Process,
  Q: Process
{
  type Value =
    Either <
      Receiver < P::Value >,
      Receiver < Q::Value >
    >;
}

impl < P, Q, R >
  AlgebraT < R > for
  InternalChoice < P, Q >
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
    InternalChoice <
      < P as
        AlgebraT < R >
      > :: Algebra,
      < Q as
        AlgebraT < R >
      > :: Algebra
    >;
}