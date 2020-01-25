use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base as base;
use crate::process as process;

use base::{ Process };
use process::fix::{ ProcessAlgebra };

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
    Sender < T >,
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

impl < T, P, R >
  ProcessAlgebra < R > for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : ProcessAlgebra < R >
{
  type ToProcess =
    ReceiveValue <
      T,
      < P as
        ProcessAlgebra < R >
      > :: ToProcess
    >;
}
