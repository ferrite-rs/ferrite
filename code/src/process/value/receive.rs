use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

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
  type Value = (
    Sender < Val < T > >,
    Receiver < P::Value >
  );
}

impl
  < T, P >
  base::public::Protocol for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{ }

impl < A, T, P >
  TyApp < A > for
  ReceiveValue < T, P >
where
  P : TyApp < A >,
{
  type Type =
    ReceiveValue <
      T,
      P :: Type
    >;
}
