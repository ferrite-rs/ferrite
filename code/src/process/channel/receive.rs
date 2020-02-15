use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

pub struct ReceiveChannel
  < P, Q >
{
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl
  < P, Q >
  Protocol for
  ReceiveChannel < P, Q >
where
  P: Protocol,
  Q: Protocol
{
  type Value =
    Sender < (
      Receiver < P :: Value >,
      Sender < Q :: Value >
    ) >;
}

impl
  < P, Q >
  base::public::Protocol for
  ReceiveChannel < P, Q >
where
  P: base::public::Protocol,
  Q: base::public::Protocol
{ }


impl < A, P, Q >
  TyApp < A > for
  ReceiveChannel < P, Q >
where
  P : TyApp < A >,
  Q : TyApp < A >,
{
  type Type =
    ReceiveChannel <
      P :: Type,
      Q :: Type
    >;
}
