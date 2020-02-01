use std::marker::PhantomData;

use crate::base::*;
use super::data::*;

pub struct InternalChoice < Row >
  ( PhantomData < Row > );

impl < Row >
  Process for
  InternalChoice < Row >
where
  Row : Send + 'static,
  Row : SumRow < ReceiverCon >,
  < Row as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
{
  type Value = Row :: Field;
}