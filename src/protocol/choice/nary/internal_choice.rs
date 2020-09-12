use crate::base::*;
use super::data::*;

pub struct InternalChoice < Row >
where
  Row : SumRow < ReceiverApp >,
{ pub (crate) field :
    Row::Field
}

impl < Row >
  Protocol for
  InternalChoice < Row >
where
  Row : Send + 'static,
  Row : SumRow < ReceiverApp >,
  Row::Field : Send,
{ }

impl < Row, A >
  TypeApp < A > for
  InternalChoice < Row >
where
  Row : SumRow < ReceiverApp >,
  Row : TypeApp < A >,
  Row::Applied : SumRow < ReceiverApp >,
{
  type Applied =
    InternalChoice <
      Row::Applied
    >;
}
