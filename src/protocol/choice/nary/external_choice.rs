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


impl < Row1, Row2, A >
  TypeApp < A > for
  ExternalChoice < Row1 >
where
  Row1 : RowApp <
    (), A,
    Applied = Row2,
  >,
  Row1 : RowApp <
    ReceiverApp, A,
    Applied = Row2
  >,
  Row2 : SumRow < () >,
  Row2 : SumRow < ReceiverApp >,
{
  type Applied =
    ExternalChoice <
      < Row1 as
        RowApp < (), A >
      > ::Applied
    >;
}
