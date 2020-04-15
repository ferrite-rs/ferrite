use crate::base::*;
use super::data::*;

pub struct InternalChoice < Row >
where
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverCon >,
  < Row::Canon as
    SumRow < ReceiverCon >
  >  :: Field
    : Send
{ pub (crate) field :
    < Row::Canon as
      SumRow < ReceiverCon >
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
    SumRow < ReceiverCon >,
  < Row::Canon as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
{ }
