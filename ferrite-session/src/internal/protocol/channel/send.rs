use crate::internal::base::*;

pub struct SendChannel<A, B>(
  pub(crate) ReceiverOnce<A>,
  pub(crate) ReceiverOnce<B>,
);

impl<A, B> Protocol for SendChannel<A, B>
where
  A: Protocol,
  B: Protocol,
{
  type ConsumerEndpoint =
    (ReceiverOnce<A::ConsumerEndpoint>, B::ConsumerEndpoint);
  type ProviderEndpoint =
    (SenderOnce<A::ConsumerEndpoint>, B::ProviderEndpoint);
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
