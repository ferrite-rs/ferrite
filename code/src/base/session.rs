
use crate::base::process::*;
use crate::base::processes::*;
use std::pin::Pin;
use std::future::{ Future };
use async_std::sync::Sender;

/// A session builder is a consumer for the given list of
/// input processes and output a process with given Out type.
pub type Session < P > = PartialSession < (), P >;

pub struct PartialSession
  < I, P >
where
  P: Process,
  I: Processes
{
  pub builder : Box <
    dyn FnOnce( I::Values, Sender < P::Value > )
      -> Pin < Box < dyn Future < Output=() > + Send > >
    + Send
  >
}