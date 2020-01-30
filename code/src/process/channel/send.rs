use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyCon, Process };

pub struct SendChannel < P, Q >  {
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl < P, Q >
  Process for
  SendChannel < P, Q >
where
  P: Process,
  Q: Process
{
  type Value = (
    Receiver< P::Value >,
    Receiver< Q::Value >
  );
}

impl
  < P, Q >
  base::public::Process for
  SendChannel < P, Q >
where
  P: base::public::Process,
  Q: base::public::Process
{ }

impl < A, P, Q >
  TyCon < A > for
  SendChannel < P, Q >
where
  P : TyCon < A >,
  Q : TyCon < A >,
{
  type Type =
  SendChannel <
      P :: Type,
      Q :: Type
    >;
}
