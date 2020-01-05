use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;
use crate::process as process;

use base::{ Process };
use process::fix::{ ProcessAlgebra };

pub struct SendValue < T, P >
{
  value: PhantomData<T>,
  process: PhantomData<P>
}

impl < T, P > Process for SendValue < T, P >
where
  T : Send,
  P : Process
{
  type Value = (
    T,
    Receiver < P::Value >
  );
}

impl
  < T, P >
  base::public::Process for
  SendValue < T, P >
where
  T : Send,
  P : Process
{ }

impl < T, P, R >
  ProcessAlgebra < R > for
  SendValue < T, P >
where
  T : Send,
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