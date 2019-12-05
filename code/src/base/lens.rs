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

pub trait ProcessLens < S, T, D, P1, P2 >
where
  S : Processes,
  T : Processes,
  D : Processes,
  P1 : ProcessNode,
  P2 : ProcessNode
{
  fn split_channels (
    channels : < S as Processes > :: Values
  ) ->
    ( < P1 as ProcessNode > :: NodeValue,
      < D as Processes
      > :: Values
    );

  fn merge_channels (
    receiver : < P2 as ProcessNode > :: NodeValue,
    channels :
      < D as Processes >
      :: Values
  ) ->
    < T as Processes > :: Values;
}
