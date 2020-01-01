
use async_std::sync::{ Sender, Receiver };

use crate::process::{
  SendValue,
  InternalChoice,
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

impl < P, Q, R >
  SharedAlgebra < R > for
  InternalChoice < P, Q >
where
  P : SharedAlgebra < R >,
  Q : SharedAlgebra < R >,
{
  type ToProcess =
    InternalChoice <
      P :: ToProcess,
      Q :: ToProcess
    >;
}
