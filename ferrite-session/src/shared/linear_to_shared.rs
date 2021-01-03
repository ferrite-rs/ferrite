
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

impl < F > serde::Serialize
  for LinearToShared < F >
where
  F : SharedRecApp < SharedToLinear < F > >,
  F::Applied:
    serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.linear.serialize(serializer)
  }
}

impl < 'a, F > serde::Deserialize<'a>
for LinearToShared < F >
where
  F : SharedRecApp < SharedToLinear < F > >,
  F::Applied:
    serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    let linear =
      < F::Applied
      >::deserialize(deserializer)?;

    Ok(LinearToShared{linear})
  }
}
