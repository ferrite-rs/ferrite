use crate::base::{ Protocol, Z };

use crate::protocol::{
  ExternalChoice,
  InternalChoice,

  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

pub trait SharedTypeApp < X >
{
  type Applied;
}

impl < X >
  SharedTypeApp < X > for
  Z
{
  type Applied = X;
}

impl < T, A, X >
  SharedTypeApp < X > for
  SendValue < T, A >
where
  T : Send + 'static,
  A : SharedTypeApp < X >
{
  type Applied =
    SendValue <
      T,
      A :: Applied
    >;
}

impl < T, A, X >
  SharedTypeApp < X > for
  ReceiveValue < T, A >
where
  T : Send + 'static,
  A : SharedTypeApp < X >
{
  type Applied =
    ReceiveValue <
      T,
      A :: Applied
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

impl < A, B, X >
  SharedTypeApp < X > for
  ReceiveChannel < A, B >
where
  B : SharedTypeApp < X >,
{
  type Applied =
    ReceiveChannel <
      A,
      B :: Applied
    >;
}
