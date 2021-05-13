use base::Protocol;

use crate::internal::base;

pub trait Wrapper
{
  type Unwrap: Protocol;
}

pub struct Wrap<T>
where
  T: Wrapper,
{
  pub(crate) unwrap: Box<T::Unwrap>,
}

impl<T> Protocol for Wrap<T>
where
  T: Wrapper,
  T: Send + 'static,
{
}
