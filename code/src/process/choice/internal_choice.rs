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

impl
  < P, Q >
  base::public::Protocol for
  InternalChoice < P, Q >
where
  P: base::public::Protocol,
  Q: base::public::Protocol
{ }

impl < A, X, Y >
  TyApp < A > for
  InternalChoice < X, Y >
where
  X : TyApp < A >,
  Y : TyApp < A >,
{
  type Type =
    InternalChoice <
      X :: Type,
      Y :: Type
    >;
}
