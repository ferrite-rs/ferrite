use std::marker::PhantomData;

use crate::base::*;
use super::data::*;

pub struct InternalChoice < Row >
  ( PhantomData < Row > );

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
{
  type Payload =
    < Row::Canon as
      SumRow < ReceiverCon >
    >  :: Field;
}
