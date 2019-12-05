use crate::base::*;
use crate::process::*;
use crate::shared::process::*;
use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

pub struct ReleaseF {}

pub struct SendValueF < T, P >
{
  t : PhantomData < T >,
  p : PhantomData < P >
}

pub struct InternalChoiceF < P, Q >
{
  p : PhantomData < P >,
  q : PhantomData < Q >
}

impl < F >
  SharedProcess for
  LinearToShared < F >
where
  F : ProcessAlgebra < F >
{
  type SharedValue =
    < < F as ProcessAlgebra < F > >
      :: ToProcess
      as Process
    > :: Value;
}

impl < F >
  Process for
  SharedToLinear < F >
{
  type Value = ();
}

impl < F >
  Process for
  Lock < F >
where
  F : ProcessAlgebra < F >
{
  type Value =
    Sender <
      Receiver<
        < < F as ProcessAlgebra < F > >
          :: ToProcess
          as Process
        > :: Value
      >
    >;
}

impl < F >
  ProcessAlgebra < F > for
  ReleaseF
{
  type ToProcess = SharedToLinear < F >;
}

impl < T, P, R >
  ProcessAlgebra < R > for
  SendValueF < T, P >
where
  T : Send,
  P : ProcessAlgebra < R >
{
  type ToProcess =
    SendValue <
      T,
      P :: ToProcess
    >;
}

impl < P, Q, R >
  ProcessAlgebra < R > for
  InternalChoiceF < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : ProcessAlgebra < R >,
{
  type ToProcess =
    InternalChoice <
      P :: ToProcess,
      Q :: ToProcess
    >;
}
