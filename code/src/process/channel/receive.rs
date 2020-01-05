use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base as base;

use base::{ Process };
use crate::process::fix::{ ProcessAlgebra };

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

impl < P, Q, R >
  ProcessAlgebra < R > for
  ReceiveChannel < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : ProcessAlgebra < R >,
  < P as
    ProcessAlgebra < R >
  > :: ToProcess
    : Process,
  < Q as
    ProcessAlgebra < R >
  > :: ToProcess
    : Process
{
  type ToProcess =
    ReceiveChannel <
      < P as
        ProcessAlgebra < R >
      > :: ToProcess,
      < Q as
        ProcessAlgebra < R >
      > :: ToProcess
    >;
}