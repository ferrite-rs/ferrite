
use crate::base::{ Protocol };
use super::fix::{ SharedRecApp };
use super::protocol::{ SharedProtocol };
use super::shared_to_linear::{ SharedToLinear };

pub struct LinearToShared < F >
where
  F : SharedRecApp < SharedToLinear < F > >
{ pub (crate)
    linear : F :: Applied
}

impl < F >
  SharedProtocol for
  LinearToShared < F >
where
  F : Protocol,
  F : SharedRecApp < SharedToLinear < F > >,
  F::Applied : Protocol
{ }
