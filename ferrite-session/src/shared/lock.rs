use crate::base::*;

use super::fix::{ SharedRecApp };
use super::shared_session::{ SharedPayload };
use super::linear_to_shared::{ LinearToShared };
use super::shared_to_linear::{ SharedToLinear };

// use ipc_channel::ipc;

pub struct Lock < F >
where
  F : SharedRecApp < SharedToLinear < F > >
{
  pub (crate) unlock:
    Receiver <
      SharedPayload < LinearToShared < F > >
    >,

  pub (crate) release:
    SenderOnce < () >,
}

impl < F >
  Protocol for
  Lock < F >
where
  F : Protocol,
  F : SharedRecApp < SharedToLinear < F > >,
  F::Applied : Protocol
{ }
