use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

pub struct SendChannel < P, Q >  {
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl < P, Q >
  Protocol for
  SendChannel < P, Q >
where
  P: Protocol,
  Q: Protocol
{
  type Payload = (
    Receiver< P::Payload >,
    Receiver< Q::Payload >
  );
}

impl < A, P, Q >
  TyApp < A > for
  SendChannel < P, Q >
where
  P : TyApp < A >,
  Q : TyApp < A >,
{
  type Type =
  SendChannel <
      P :: Type,
      Q :: Type
    >;
}
