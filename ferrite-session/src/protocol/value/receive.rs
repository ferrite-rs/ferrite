use serde;
use crate::base::*;

pub struct ReceiveValue
  < T, A >
( pub (crate)
  Sender < (
    T,
    Sender < A >
  ) >
);

impl
  < T, A >
  Protocol for
  ReceiveValue < T, A >
where
  T : Send + 'static,
  A : Protocol
{ }

impl < X, T, A >
  RecApp < X > for
  ReceiveValue < T, A >
where
  X : Send + 'static,
  T : Send + 'static,
  A : RecApp < A >,
{
  type Applied =
    ReceiveValue <
      T,
      A :: Applied
    >;
}

impl < T, A > serde::Serialize
  for ReceiveValue < T, A >
where
  T: Send + 'static,
  A: Send + 'static,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
  A: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl < 'a, T, A > serde::Deserialize <'a>
  for ReceiveValue < T, A >
where
  T: Send + 'static,
  A: Send + 'static,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
  A: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    let sender =
      < Sender < (T, Sender < A > ) >
      >::deserialize(deserializer)?;

    Ok(ReceiveValue(sender))
  }
}
