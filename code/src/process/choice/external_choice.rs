use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyCon, Process };

use super::data::{ Choice, Either };

pub struct ExternalChoice
  < P, Q >
{
  p: PhantomData < P >,
  q: PhantomData < Q >
}

impl
  < P, Q >
  Process for
  ExternalChoice < P, Q >
where
  P: Process,
  Q: Process
{
  type Value = Box<
    dyn FnOnce(Choice) ->
      Either <
        Receiver < P::Value >,
        Receiver < Q::Value >
      >
    + Send >;
}

impl
  < P, Q >
  base::public::Process for
  ExternalChoice < P, Q >
where
  P: base::public::Process,
  Q: base::public::Process
{ }

impl < A, T, X, Y >
  TyCon < A > for
  Box <
    dyn FnOnce (T) ->
      Either < X, Y >
    + Send
  >
where
  X : TyCon < A >,
  Y : TyCon < A >,
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
  TyCon < A > for
  ExternalChoice < P, Q >
where
  P : TyCon < A >,
  Q : TyCon < A >,
{
  type Type =
  ExternalChoice <
      P :: Type,
      Q :: Type
    >;
}