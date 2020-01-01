use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };
use crate::process::fix::{ ProcessAlgebra };

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

impl < P, Q, R >
  ProcessAlgebra < R > for
  SendChannel < P, Q >
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
    SendChannel <
      < P as
        ProcessAlgebra < R >
      > :: ToProcess,
      < Q as
        ProcessAlgebra < R >
      > :: ToProcess
    >;
}