use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

use super::data::{ Choice, Either };

pub struct ExternalChoice
  < P, Q >
{
  p: PhantomData < P >,
  q: PhantomData < Q >
}

impl
  < P, Q >
  Protocol for
  ExternalChoice < P, Q >
where
  P: Protocol,
  Q: Protocol
{
  type Payload = Box<
    dyn FnOnce(Choice) ->
      Either <
        Receiver < P::Payload >,
        Receiver < Q::Payload >
      >
    + Send >;
}

impl
  < P, Q >
  base::public::Protocol for
  ExternalChoice < P, Q >
where
  P: base::public::Protocol,
  Q: base::public::Protocol
{ }

impl < A, T, X, Y >
  TyApp < A > for
  Box <
    dyn FnOnce (T) ->
      Either < X, Y >
    + Send
  >
where
  X : TyApp < A >,
  Y : TyApp < A >,
{
  type Type =
    Box <
      dyn FnOnce (T) ->
        Either <
          X :: Type,
          Y :: Type
        >
      + Send
    >;
}

impl < A, P, Q >
  TyApp < A > for
  ExternalChoice < P, Q >
where
  P : TyApp < A >,
  Q : TyApp < A >,
{
  type Type =
  ExternalChoice <
      P :: Type,
      Q :: Type
    >;
}
