use crate::internal::base::*;

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
  type ConsumerEndpoint = ReceiverOnce<Wrap<T>>;
  type ProviderEndpoint = SenderOnce<Wrap<T>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    once_channel()
  }
}
