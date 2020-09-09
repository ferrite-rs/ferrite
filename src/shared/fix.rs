use crate::base::{ Protocol, Z };

use crate::protocol::{
  SendValue,
  ReceiveValue,

  SendChannel,
  ReceiveChannel,
};

// use crate::protocol::choice::nary:: {
//   Iso,
//   SumRow,
//   ReceiverApp,
//   ExternalChoice,
//   InternalChoice,
// };

use crate::protocol::choice::nary::either:: {
  Either
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

// impl < Row1, Canon1, Row2, Canon2, A >
//   SharedTypeApp < A > for
//   ExternalChoice < Row1 >
// where
//   Row1 : SharedTypeApp < A, Applied = Row2 >,
//   Row1 : Iso < Canon = Canon1 >,
//   Row1 :
//     Send + 'static,
//   Canon1 :
//     SumRow < ReceiverApp >,
//   Canon1 :
//     SumRow < () >,
//   < Canon1 as
//     SumRow < ReceiverApp >
//   >  :: Field
//     : Send,
//   < Canon1 as
//     SumRow < () >
//   >  :: Field
//     : Send,
//   Row2 : Iso < Canon = Canon2 >,
//   Row2 :
//     Send + 'static,
//   Canon2 :
//     SumRow < ReceiverApp >,
//   Canon2 :
//     SumRow < () >,
//   < Canon2 as
//     SumRow < ReceiverApp >
//   >  :: Field
//     : Send,
//   < Canon2 as
//     SumRow < () >
//   >  :: Field
//     : Send,
// {
//   type Applied =
//     ExternalChoice <
//       Row2
//     >;
// }

// impl < Row1, Canon1, Row2, Canon2, A >
//   SharedTypeApp < A > for
//   InternalChoice < Row1 >
// where
//   Row1 : SharedTypeApp < A, Applied = Row2 >,
//   Row1 : Iso < Canon = Canon1 >,
//   Row1 :
//     Send + 'static,
//   Canon1 :
//     SumRow < ReceiverApp >,
//   < Canon1 as
//     SumRow < ReceiverApp >
//   >  :: Field
//     : Send,
//   Row2 : Iso < Canon = Canon2 >,
//   Row2 :
//     Send + 'static,
//   Canon2 :
//     SumRow < ReceiverApp >,
//   < Canon2 as
//     SumRow < ReceiverApp >
//   >  :: Field
//     : Send,
// {
//   type Applied =
//     InternalChoice <
//       Row2
//     >;
// }
