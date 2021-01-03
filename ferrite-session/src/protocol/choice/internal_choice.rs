use serde;
use crate::base::*;
use super::utils::*;
use crate::functional::row::*;

pub struct InternalChoice < Row >
where
  Row : RowCon,
{ pub (crate) field :
    AppliedSum < Row, ReceiverF >
}

impl < Row >
  Protocol for
  InternalChoice < Row >
where
  Row : Send + 'static,
  Row : RowCon,
{ }

impl < Row, A >
  RecApp < A > for
  InternalChoice < Row >
where
  Row : RowCon,
  Row : RecApp < A >,
  Row::Applied : RowCon,
{
  type Applied =
    InternalChoice <
      Row::Applied
    >;
}

impl < Row > serde::Serialize
  for InternalChoice < Row >
where
  Row : RowCon,
  AppliedSum < Row, ReceiverF >:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.field.serialize(serializer)
  }
}

impl < 'a, Row > serde::Deserialize<'a>
  for InternalChoice < Row >
where
  Row : RowCon,
  AppliedSum < Row, ReceiverF >:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{

  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    let field =
      < AppliedSum < Row, ReceiverF >
      >::deserialize(deserializer)?;

    Ok(InternalChoice{field})
  }
}
