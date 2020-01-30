use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyCon, Process };

use super::data::{ Either };

/*
  data InternalChoice p q = InternalChoice
 */
pub struct InternalChoice < P, Q >
{
  p : PhantomData < P >,
  q : PhantomData < Q >
}

impl
  < P, Q >
  Process for
  InternalChoice < P, Q >
where
  P: Process,
  Q: Process
{
  type Value =
    Either <
      Receiver < P::Value >,
      Receiver < Q::Value >
    >;
}

impl
  < P, Q >
  base::public::Process for
  InternalChoice < P, Q >
where
  P: base::public::Process,
  Q: base::public::Process
{ }

impl < A, X, Y >
  TyCon < A > for
  InternalChoice < X, Y >
where
  X : TyCon < A >,
  Y : TyCon < A >,
{
  type Type =
    InternalChoice <
      X :: Type,
      Y :: Type
    >;
}