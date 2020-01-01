
use async_std::sync::{ Sender, Receiver };

use crate::process::{
  ProcessAlgebra,

  ExternalChoice,
  InternalChoice,

  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

use super::process::{
  Lock,
  Release,
  SharedProcess,
  SharedAlgebra,
  LinearToShared,
  SharedToLinear,
};

use crate::base::{
  Process,
};

impl < F >
  SharedProcess for
  LinearToShared < F >
where
  F : SharedAlgebra < F >
{
  type SharedValue =
    < < F as SharedAlgebra < F > >
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
  F : SharedAlgebra < F >
{
  type Value =
    Sender <
      Receiver<
        < < F as SharedAlgebra < F > >
          :: ToProcess
          as Process
        > :: Value
      >
    >;
}

impl < F >
  SharedAlgebra < F > for
  Release
{
  type ToProcess = SharedToLinear < F >;
}

impl < T, P, R >
  SharedAlgebra < R > for
  SendValue < T, P >
where
  T : Send,
  P : SharedAlgebra < R >
{
  type ToProcess =
    SendValue <
      T,
      P :: ToProcess
    >;
}

impl < T, P, R >
  SharedAlgebra < R > for
  ReceiveValue < T, P >
where
  T : Send,
  P : SharedAlgebra < R >
{
  type ToProcess =
    ReceiveValue <
      T,
      P :: ToProcess
    >;
}

impl < P, Q, R >
  SharedAlgebra < R > for
  InternalChoice < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : SharedAlgebra < R >,
{
  type ToProcess =
    InternalChoice <
      P :: ToProcess,
      Q :: ToProcess
    >;
}

impl < P, Q, R >
  SharedAlgebra < R > for
  ExternalChoice < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : SharedAlgebra < R >,
{
  type ToProcess =
    ExternalChoice <
      P :: ToProcess,
      Q :: ToProcess
    >;
}

impl < P, Q, R >
  SharedAlgebra < R > for
  SendChannel < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : SharedAlgebra < R >,
{
  type ToProcess =
    SendChannel <
      P :: ToProcess,
      Q :: ToProcess
    >;
}

impl < P, Q, R >
  SharedAlgebra < R > for
  ReceiveChannel < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : SharedAlgebra < R >,
{
  type ToProcess =
    ReceiveChannel <
      P :: ToProcess,
      Q :: ToProcess
    >;
}
