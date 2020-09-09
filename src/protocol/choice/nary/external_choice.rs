use crate::base::*;
use super::data::*;
use async_std::sync::{ Sender };

pub struct ExternalChoice < Row >
where
  Row : Iso,
  Row : Send + 'static,
  Row::Canon :
    SumRow < () >,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < () >
  >  :: Field
    : Send,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
{ pub sender :
    Sender <
      ( < Row::Canon as
          SumRow < () >
        > :: Field,
        Sender <
          < Row::Canon as
            SumRow < ReceiverApp >
          > :: Field
        >
      )
    >
}

impl < Row >
  Protocol for
  ExternalChoice < Row >
where
  Row : Iso,
  Row : Send + 'static,
  Row::Canon : SumRow < () >,
  Row::Canon : SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < () >
  >  :: Field
    : Send,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
{ }


// impl < Row1, Canon1, Row2, Canon2, A >
//   TypeApp < A > for
//   ExternalChoice < Row1 >
// where
//   Row1 : TypeApp < A, Applied = Row2 >,
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
