use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

use super::data::{ Either };

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
  Protocol for
  InternalChoice < P, Q >
where
  P: Protocol,
  Q: Protocol
{
  type Payload =
    Either <
      Receiver < P::Payload >,
      Receiver < Q::Payload >
    >;
}

impl < A, X, Y >
  TyApp < A > for
  InternalChoice < X, Y >
where
  X : TyApp < A >,
  Y : TyApp < A >,
{
  type Applied =
    InternalChoice <
      X :: Applied,
      Y :: Applied
    >;
}
