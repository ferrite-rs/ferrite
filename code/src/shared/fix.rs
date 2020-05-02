use crate::base::{ Protocol, Z };

use crate::process::{
  ExternalChoice,
  InternalChoice,

  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

pub trait SharedTypeApp < R >
{
  type Applied;
}

impl < R >
  SharedTypeApp < R > for
  Z
{
  type Applied = R;
}

impl < T, P, R >
  SharedTypeApp < R > for
  SendValue < T, P >
where
  T : Send + 'static,
  P : SharedTypeApp < R >
{
  type Applied =
    SendValue <
      T,
      P :: Applied
    >;
}

impl < T, P, R >
  SharedTypeApp < R > for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : SharedTypeApp < R >
{
  type Applied =
    ReceiveValue <
      T,
      P :: Applied
    >;
}

impl < P, Q, R >
  SharedTypeApp < R > for
  InternalChoice < P, Q >
where
  P : SharedTypeApp < R >,
  Q : SharedTypeApp < R >,
{
  type Applied =
    InternalChoice <
      P :: Applied,
      Q :: Applied
    >;
}

impl < P, Q, R >
  SharedTypeApp < R > for
  ExternalChoice < P, Q >
where
  P : SharedTypeApp < R >,
  Q : SharedTypeApp < R >,
{
  type Applied =
    ExternalChoice <
      P :: Applied,
      Q :: Applied
    >;
}

impl < P, Q, R >
  SharedTypeApp < R > for
  SendChannel < P, Q >
where
  P : Protocol,
  Q : SharedTypeApp < R >,
{
  type Applied =
    SendChannel <
      P,
      Q :: Applied
    >;
}

impl < P, Q, R >
  SharedTypeApp < R > for
  ReceiveChannel < P, Q >
where
  Q : SharedTypeApp < R >,
{
  type Applied =
    ReceiveChannel <
      P,
      Q :: Applied
    >;
}
