
use crate::base::{ Process, Z };

use crate::process::{
  ExternalChoice,
  InternalChoice,

  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

use super::process::{
  SharedTyCon,
  SharedToLinear,
};

impl < F >
  SharedTyCon < F > for
  Z
where
  F : Send + 'static
{
  type ToProcess = SharedToLinear < F >;
}

impl < T, P, R >
  SharedTyCon < R > for
  SendValue < T, P >
where
  T : Send + 'static,
  P : SharedTyCon < R >
{
  type ToProcess =
    SendValue <
      T,
      P :: ToProcess
    >;
}

impl < T, P, R >
  SharedTyCon < R > for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : SharedTyCon < R >
{
  type ToProcess =
    ReceiveValue <
      T,
      P :: ToProcess
    >;
}

impl < P, Q, R >
  SharedTyCon < R > for
  InternalChoice < P, Q >
where
  P : SharedTyCon < R >,
  Q : SharedTyCon < R >,
{
  type ToProcess =
    InternalChoice <
      P :: ToProcess,
      Q :: ToProcess
    >;
}

impl < P, Q, R >
  SharedTyCon < R > for
  ExternalChoice < P, Q >
where
  P : SharedTyCon < R >,
  Q : SharedTyCon < R >,
{
  type ToProcess =
    ExternalChoice <
      P :: ToProcess,
      Q :: ToProcess
    >;
}

impl < P, Q, R >
  SharedTyCon < R > for
  SendChannel < P, Q >
where
  P : Process,
  Q : SharedTyCon < R >,
{
  type ToProcess =
    SendChannel <
      P,
      Q :: ToProcess
    >;
}

impl < P, Q, R >
  SharedTyCon < R > for
  ReceiveChannel < P, Q >
where
  P : Process,
  Q : SharedTyCon < R >,
{
  type ToProcess =
    ReceiveChannel <
      P,
      Q :: ToProcess
    >;
}
