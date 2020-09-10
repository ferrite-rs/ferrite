use crate::base::*;
use super::data::*;

pub struct InternalChoice < Row >
where
  Row : SumRow < ReceiverApp >,
{ pub (crate) field :
    < Row as
      SumRow < ReceiverApp >
    >  :: Field
}

impl < Row >
  Protocol for
  InternalChoice < Row >
where
  Row : Send + 'static,
  Row : SumRow < ReceiverApp >,
  < Row as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
{ }

impl < Row1, A >
  TypeApp < A > for
  InternalChoice < Row1 >
where
  Row1 : RowApp < ReceiverApp, A >,
{
  type Applied =
    InternalChoice <
      Row1::Applied
    >;
}
