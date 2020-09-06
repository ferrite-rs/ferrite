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
