use async_std::sync::Receiver;

use crate::base::process::{ Protocol };
use crate::base::processes::{ Context };

pub trait Slot {
  type SlotValue : Send;
}

impl < P > Slot for P
where
  P : Protocol
{
  type SlotValue = Receiver <
    < P as Protocol > :: Value
  >;
}

pub struct Empty { }

impl Slot for Empty {
  type SlotValue = ();
}

pub trait ContextLens < I, P1, P2 >
where
  I : Context,
  P1 : Slot,
  P2 : Slot,
{
  type Deleted : Context + 'static;
  type Target : Context + 'static;

  fn split_channels (
    channels :
      < I as Context > :: Values
  ) ->
    ( < P1 as Slot > :: SlotValue,
      < Self::Deleted
        as Context
      > :: Values
    );

  fn merge_channels (
    receiver : < P2 as Slot > :: SlotValue,
    channels :
      < Self::Deleted
        as Context >
      :: Values
  ) ->
    < Self::Target
      as Context
    > :: Values;
}
