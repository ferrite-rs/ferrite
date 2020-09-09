use crate::base::*;
use super::data::*;

pub struct InternalChoice < Row >
where
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send
{ pub (crate) field :
    < Row::Canon as
      SumRow < ReceiverApp >
    >  :: Field
}

impl < Row >
  Protocol for
  InternalChoice < Row >
where
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
{ }

// impl < Row1, Canon1, Row2, Canon2, A >
//   TypeApp < A > for
//   InternalChoice < Row1 >
// where
//   Row1 : TypeApp < A, Applied = Row2 >,
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
