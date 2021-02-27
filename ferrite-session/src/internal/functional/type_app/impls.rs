use serde;

use super::{
  structs::*,
  traits::*,
};

impl<F, X, T> serde::Serialize for App<F, X>
where
  X : 'static,
  T : Send + 'static,
  F : TypeApp<X, Applied = T>,
  T : serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(
    &self,
    serializer : S,
  ) -> Result<S::Ok, S::Error>
  where
    S : serde::Serializer,
  {
    self
      .applied
      .as_ref()
      .get_applied_borrow()
      .serialize(serializer)
  }
}

impl<'a, F, X, T> serde::Deserialize<'a> for App<F, X>
where
  X : 'static,
  T : Send + 'static,
  F : TypeApp<X, Applied = T>,
  T : serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer : D) -> Result<Self, D::Error>
  where
    D : serde::Deserializer<'a>,
  {
    let applied = T::deserialize(deserializer)?;

    Ok(wrap_type_app(applied))
  }
}

impl<T, F, A> HasTypeApp<F, A> for T
where
  F : 'static,
  A : 'static,
  T : Send + 'static,
  F : TypeApp<A, Applied = T>,
{
  fn get_applied(self: Box<T>) -> Box<T>
  {
    self
  }

  fn get_applied_borrow<'a>(&'a self) -> &'a F::Applied
  where
    F : TypeApp<A>,
  {
    self
  }
}

impl TyCon for () {}

impl<X> TyCon for Const<X> where X : 'static {}

impl<A> TypeApp<A> for ()
where
  A : 'static,
{
  type Applied = ();
}

impl<X, A> TypeApp<A> for Const<X>
where
  A : 'static,
  X : Send + 'static,
{
  type Applied = X;
}
