use crate::internal::base::protocol::Protocol;

pub trait RecApp<A>: Sized
{
  type Applied: Send;
}

pub trait HasRecApp<F, C>: Send
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F: RecApp<C>;
}

pub trait SharedRecApp<X>
{
  type Applied;
}

pub trait HasRecEndpoint<F, C>: Send + 'static
{
  fn get_applied(
    self: Box<Self>
  ) -> Box<<F::Applied as Protocol>::ClientEndpoint>
  where
    F: RecApp<C>,
    F::Applied: Protocol;
}
