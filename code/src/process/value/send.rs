use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use super::data::Val;
use crate::base as base;

use base::{ TyApp, Protocol };

pub struct SendValue < T, P >
{
  value: PhantomData<T>,
  process: PhantomData<P>
}

impl < T, P > Protocol for SendValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{
  type Payload = (
    Val < T >,
    Receiver < P::Payload >
  );
}

impl < A, T, P >
  TyApp < A > for
  SendValue < T, P >
where
  P : TyApp < A >,
{
  type Type =
    SendValue <
      T,
      P :: Type
    >;
}

impl
  < T, P >
  base::public::Protocol for
  SendValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{ }
