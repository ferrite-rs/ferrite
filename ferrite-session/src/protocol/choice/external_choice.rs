use crate::base::*;
use super::utils::*;
use crate::functional::row::*;

pub struct ExternalChoice < Row >
where
  Row : RowCon,
{ pub sender :
    SenderOnce <
      ( AppliedSum < Row, () >,
        SenderOnce <
          AppliedSum < Row, ReceiverF >
        >
      )
    >
}

impl < Row >
  Protocol for
  ExternalChoice < Row >
where
  Row : Send + 'static,
  Row : RowCon,
{ }

impl < Row, A >
  RecApp < A > for
  ExternalChoice < Row >
where
  Row : RecApp < A >,
  Row : RowCon,
  Row::Applied : RowCon,
{
  type Applied =
    ExternalChoice <
      Row::Applied
    >;
}

impl < Row > serde::Serialize
  for ExternalChoice < Row >
where
  Row : RowCon,
  AppliedSum < Row, () >:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
  AppliedSum < Row, ReceiverF >:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.sender.serialize(serializer)
  }
}

impl < 'a, Row > serde::Deserialize<'a>
  for ExternalChoice < Row >
where
  Row : RowCon,
  AppliedSum < Row, () >:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
  AppliedSum < Row, ReceiverF >:
    Send + 'static
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{

  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    let sender =
      < SenderOnce <
          ( AppliedSum < Row, () >,
            SenderOnce <
              AppliedSum < Row, ReceiverF >
            >
          ) >
      >::deserialize(deserializer)?;

    Ok(ExternalChoice{sender})
  }
}
