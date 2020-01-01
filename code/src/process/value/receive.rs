use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base::{ Process };
use crate::process::fix::{ ProcessAlgebra };

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
  T : Send,
  P : Process
{
  type Value = (
    Sender < T >,
    Receiver < P::Value >
  );
}

impl < T, P, R >
  ProcessAlgebra < R > for
  ReceiveValue < T, P >
where
  T : Send,
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
