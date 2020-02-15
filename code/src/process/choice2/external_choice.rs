use std::marker::PhantomData;

use crate::base::*;
use super::data::*;
use async_std::sync::{ Sender };

pub struct ExternalChoice < Row >
  ( PhantomData < Row > );

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
{
  type Value =
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
    >;
}