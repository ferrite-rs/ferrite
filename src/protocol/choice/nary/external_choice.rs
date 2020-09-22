use crate::base::*;
use super::data::*;
use async_std::sync::{ Sender };

pub struct ExternalChoice < Row >
where
  Row : SumRow < () >,
  Row : SumRow < ReceiverApp >,
{ pub sender :
    Sender <
      ( < Row as
          SumRow < () >
        > :: Field,
        Sender <
          < Row as
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
  Row : Send + 'static,
  Row : SumRow < () >,
  Row : SumRow < ReceiverApp >,
{ }

impl < Row, A >
  RecApp < A > for
  ExternalChoice < Row >
where
  Row : RecApp < A >,
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
