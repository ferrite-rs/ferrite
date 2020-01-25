
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
  Release,
  SharedAlgebra,
  SharedToLinear,
};

impl < F >
  SharedAlgebra < F > for
  Release
where
  F : Send + 'static
{
  type ToProcess = SharedToLinear < F >;
}

impl < T, P, R >
  SharedAlgebra < R > for
  SendValue < T, P >
where
  T : Send + 'static,
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
  T : Send + 'static,
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
  P : SharedAlgebra < R >,
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
  P : SharedAlgebra < R >,
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
