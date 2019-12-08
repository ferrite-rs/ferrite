use crate::base::*;
use crate::process::*;
use crate::shared::process::*;
use async_std::sync::{ Sender, Receiver };

pub struct Release {}

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
  Release
{
  type ToProcess = SharedToLinear < F >;
}

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
      P :: ToProcess
    >;
}

impl < P, Q, R >
  ProcessAlgebra < R > for
  InternalChoice < P, Q >
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
