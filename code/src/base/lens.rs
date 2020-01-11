use async_std::sync::Receiver;

use crate::base::process::{ Process };
use crate::base::processes::{ Processes };

pub trait ProcessNode {
  type NodeValue : Send;
}

impl < P > ProcessNode for P
where
  P : Process
{
  type NodeValue = Receiver <
    < P as Process > :: Value
  >;
}

pub struct Inactive { }

impl ProcessNode for Inactive {
  type NodeValue = ();
}

pub trait ProcessLens < I, P1, P2 >
where
  I : Processes,
  P1 : ProcessNode,
  P2 : ProcessNode,
{
  type Deleted : Processes + 'static;
  type Target : Processes + 'static;

  fn split_channels (
    channels :
      < I as Processes > :: Values
  ) ->
    ( < P1 as ProcessNode > :: NodeValue,
      < Self::Deleted
        as Processes
      > :: Values
    );

  fn merge_channels (
    receiver : < P2 as ProcessNode > :: NodeValue,
    channels :
      < Self::Deleted
        as Processes >
      :: Values
  ) ->
    < Self::Target
      as Processes
    > :: Values;
}