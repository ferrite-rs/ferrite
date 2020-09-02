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
    SumRow < ReceiverCon >,
  < Row::Canon as
    SumRow < () >
  >  :: Field
    : Send,
  < Row::Canon as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
{ pub sender :
    Sender <
      ( < Row::Canon as
          SumRow < () >
        >  :: Field,
        Sender <
          < Row::Canon as
            SumRow < ReceiverCon >
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
  Row::Canon : SumRow < ReceiverCon >,
  < Row::Canon as
    SumRow < () >
  >  :: Field
    : Send,
  < Row::Canon as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
{ }
