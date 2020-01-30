use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base as base;

use base::{ TyCon, Process };

pub struct ReceiveChannel
  < P, Q >
{
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl
  < P, Q >
  Process for
  ReceiveChannel < P, Q >
where
  P: Process,
  Q: Process
{
  type Value =
    Sender < (
      Receiver < P :: Value >,
      Sender < Q :: Value >
    ) >;
}

impl
  < P, Q >
  base::public::Process for
  ReceiveChannel < P, Q >
where
  P: base::public::Process,
  Q: base::public::Process
{ }


impl < A, P, Q >
  TyCon < A > for
  ReceiveChannel < P, Q >
where
  P : TyCon < A >,
  Q : TyCon < A >,
{
  type Type =
    ReceiveChannel <
      P :: Type,
      Q :: Type
    >;
}
