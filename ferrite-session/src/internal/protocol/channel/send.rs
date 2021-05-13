use crate::internal::base::*;

pub struct SendChannel<A, B>(
  pub(crate) ReceiverOnce<A>,
  pub(crate) ReceiverOnce<B>,
);

impl<P, Q> Protocol for SendChannel<P, Q>
where
  P: Protocol,
  Q: Protocol,
{
}

impl<A, P, Q> RecApp<A> for SendChannel<P, Q>
where
  P: RecApp<A>,
  Q: RecApp<A>,
{
  type Applied = SendChannel<P::Applied, Q::Applied>;
}

impl<P, Q, R> SharedRecApp<R> for SendChannel<P, Q>
where
  P: Protocol,
  Q: SharedRecApp<R>,
{
  type Applied = SendChannel<P, Q::Applied>;
}
