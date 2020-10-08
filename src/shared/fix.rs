use crate::base::{ Protocol };

use crate::functional::nat::*;
use crate::functional::row::*;

use crate::protocol::{
  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

use crate::protocol::*;

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
  Either < P, Q >
where
  P : SharedTypeApp < R >,
  Q : SharedTypeApp < R >,
{
  type Applied =
    Either <
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

impl < Row, A >
  SharedTypeApp < A > for
  InternalChoice < Row >
where
  Row : SumRow < ReceiverApp >,
  Row : SharedTypeApp < A >,
  Row::Applied : SumRow < ReceiverApp >,
{
  type Applied =
    InternalChoice <
      Row::Applied
    >;
}

impl < Row, A >
  SharedTypeApp < A > for
  ExternalChoice < Row >
where
  Row : SharedTypeApp < A >,
  Row : SumRow < () >,
  Row : SumRow < ReceiverApp >,
  Row::Applied : SumRow < () >,
  Row::Applied : SumRow < ReceiverApp >,
{
  type Applied =
    ExternalChoice <
      Row::Applied
    >;
}
