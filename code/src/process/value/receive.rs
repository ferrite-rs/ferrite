use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use super::data::Val;
use crate::base as base;

use base::{ TyCon, Process };

pub struct ReceiveValue
  < T, P >
{
  value: PhantomData < T >,
  process: PhantomData < P >
}

impl
  < T, P >
  Process for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : Process
{
  type Value = (
    Sender < Val < T > >,
    Receiver < P::Value >
  );
}

impl
  < T, P >
  base::public::Process for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : Process
{ }

impl < A, T, P >
  TyCon < A > for
  ReceiveValue < T, P >
where
  P : TyCon < A >,
{
  type Type =
    ReceiveValue <
      T,
      P :: Type
    >;
}
