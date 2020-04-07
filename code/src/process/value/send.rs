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

impl < X, T, A >
  TyApp < X > for
  SendValue < T, A >
where
  A : TyApp < X >,
{
  type Applied =
    SendValue <
      T,
      A :: Applied
    >;
}
