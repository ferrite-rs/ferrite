
use crate::base::{ Protocol, Z };

use crate::process::{
  ExternalChoice,
  InternalChoice,

  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

use super::process::{
  SharedTyApp,
  SharedToLinear,
};

impl < F >
  SharedTyApp < F > for
  Z
where
  F : Send + 'static
{
  type ToProtocol = SharedToLinear < F >;
}

impl < T, P, R >
  SharedTyApp < R > for
  SendValue < T, P >
where
  T : Send + 'static,
  P : SharedTyApp < R >
{
  type ToProtocol =
    SendValue <
      T,
      P :: ToProtocol
    >;
}

impl < T, P, R >
  SharedTyApp < R > for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : SharedTyApp < R >
{
  type ToProtocol =
    ReceiveValue <
      T,
      P :: ToProtocol
    >;
}

impl < P, Q, R >
  SharedTyApp < R > for
  InternalChoice < P, Q >
where
  P : SharedTyApp < R >,
  Q : SharedTyApp < R >,
{
  type ToProtocol =
    InternalChoice <
      P :: ToProtocol,
      Q :: ToProtocol
    >;
}

impl < P, Q, R >
  SharedTyApp < R > for
  ExternalChoice < P, Q >
where
  P : SharedTyApp < R >,
  Q : SharedTyApp < R >,
{
  type ToProtocol =
    ExternalChoice <
      P :: ToProtocol,
      Q :: ToProtocol
    >;
}

impl < P, Q, R >
  SharedTyApp < R > for
  SendChannel < P, Q >
where
  P : Protocol,
  Q : SharedTyApp < R >,
{
  type ToProtocol =
    SendChannel <
      P,
      Q :: ToProtocol
    >;
}

impl < P, Q, R >
  SharedTyApp < R > for
  ReceiveChannel < P, Q >
where
  P : Protocol,
  Q : SharedTyApp < R >,
{
  type ToProtocol =
    ReceiveChannel <
      P,
      Q :: ToProtocol
    >;
}
