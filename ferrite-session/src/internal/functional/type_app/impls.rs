use super::{
  structs::*,
  traits::*,
};

impl<'a, F, X, T> serde::Serialize for App<'a, F, X>
where
  T: Send,
  F: TypeApp<'a, X, Applied = T>,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self
      .applied
      .as_ref()
      .get_applied_borrow()
      .serialize(serializer)
  }
}

impl<'a, F, X, T: 'a> serde::Deserialize<'a> for App<'a, F, X>
where
  F: TypeApp<'a, X, Applied = T>,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    let applied = T::deserialize(deserializer)?;

    Ok(wrap_type_app(applied))
  }
}

impl<'a, T, F, A> HasTypeApp<'a, F, A> for T
where
  T: Send,
  F: TypeApp<'a, A, Applied = T>,
{
  fn get_applied(self: Box<T>) -> Box<T>
  {
    self
  }

  fn get_applied_borrow(&self) -> &F::Applied
  where
    F: TypeApp<'a, A>,
  {
    self
  }
}

impl TyCon for () {}

impl<'a, A> TypeApp<'a, A> for ()
{
  type Applied = ();
}
