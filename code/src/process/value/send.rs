use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;
use crate::process as process;

use base::{ Process };
use process::fix::{ ProcessAlgebra };
use crate::base::fix::{ TyCon };

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
    T,
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

impl < T, P, R >
  ProcessAlgebra < R > for
  SendValue < T, P >
where
  T : Send + 'static,
  P : ProcessAlgebra < R >
{
  type ToProcess =
    SendValue <
      T,
      < P as
        ProcessAlgebra < R >
      > :: ToProcess
    >;
}