use crate::base::{ Protocol };
use async_std::sync::{ Sender, Receiver };

use super::fix::{ SharedTypeApp };
use super::linear_to_shared::{ LinearToShared };
use super::shared_to_linear::{ SharedToLinear };

pub struct Lock < F >
where
  F : SharedTypeApp < SharedToLinear < F > >
{
  pub (crate) unlock:
    Receiver <
      Sender <
        Receiver<
          LinearToShared < F >
        >
      >
    >
}

impl < F >
  Protocol for
  Lock < F >
where
  F : Protocol,
  F : SharedTypeApp < SharedToLinear < F > >,
  F::Applied : Protocol
{ }
