use serde;
use crate::base::*;

use super::fix::{ SharedRecApp };
use super::linear_to_shared::{ LinearToShared };
use super::shared_to_linear::{ SharedToLinear };

pub struct Lock < F >
where
  F : SharedRecApp < SharedToLinear < F > >
{
  pub (crate) unlock:
    Receiver <
      SenderOnce <
        ReceiverOnce<
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
  F : SharedRecApp < SharedToLinear < F > >,
  F::Applied : Protocol
{ }

impl < F > serde::Serialize
  for Lock < F >
where
  F : Send + 'static
    + SharedRecApp < SharedToLinear < F > >,
  F::Applied:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.unlock.serialize(serializer)
  }
}
