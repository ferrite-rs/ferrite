use std::marker::PhantomData;
use async_std::sync::{ Sender };

use super::data::Val;
use crate::base as base;

use base::{ TyApp, Protocol };

pub struct ReceiveValue
  < T, P >
{
  value: PhantomData < T >,
  process: PhantomData < P >
}

impl
  < T, P >
  Protocol for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{
  type Payload =
    Sender <(
      Val < T >,
      Sender <
        P::Payload
      >
    )>;
}

impl < A, T, P >
  TyApp < A > for
  ReceiveValue < T, P >
where
  P : TyApp < A >,
{
  type Applied =
    ReceiveValue <
      T,
      P :: Applied
    >;
}
