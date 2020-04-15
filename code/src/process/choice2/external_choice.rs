use crate::base::*;
use super::data::*;
use async_std::sync::{ Sender };

pub struct ExternalChoice < Row >
where
  Row : Send + 'static,
  Row : SumRow < () >,
  Row : SumRow < ReceiverCon >,
  < Row as
    SumRow < () >
  >  :: Field
    : Send,
  < Row as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
{ pub (crate) sender :
    Sender <
      ( < Row as
          SumRow < () >
        >  :: Field,
        Sender <
          < Row as
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
  Row : Send + 'static,
  Row : SumRow < () >,
  Row : SumRow < ReceiverCon >,
  < Row as
    SumRow < () >
  >  :: Field
    : Send,
  < Row as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
{ }
