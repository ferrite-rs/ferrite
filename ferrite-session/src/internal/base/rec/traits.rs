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

impl<T, F, A> HasRecApp<F, A> for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  F: RecApp<A, Applied = T>,
{
  fn get_applied(self: Box<T>) -> Box<T>
  {
    self
  }
}

pub trait SharedRecApp<X>
{
  type Applied;
}
