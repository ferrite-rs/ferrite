use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use super::data::Val;
use crate::base as base;

use base::{ TyCon, Process };

pub struct SendValue < T, P >
{
  value: PhantomData<T>,
  process: PhantomData<P>
}

impl < T, P > Process for SendValue < T, P >
where
  T : Send + 'static,
  P : Process
{
  type Value = (
    Val < T >,
    Receiver < P::Value >
  );
}

impl < A, T, P >
  TyCon < A > for
  SendValue < T, P >
where
  P : TyCon < A >,
{
  type Type =
    SendValue <
      T,
      P :: Type
    >;
}

impl
  < T, P >
  base::public::Process for
  SendValue < T, P >
where
  T : Send + 'static,
  P : Process
{ }
