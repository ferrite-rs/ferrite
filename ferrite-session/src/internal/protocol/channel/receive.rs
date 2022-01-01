use crate::internal::base::*;

pub struct ReceiveChannel<A, B>(
  pub(crate) SenderOnce<(ReceiverOnce<A>, SenderOnce<B>)>,
);

impl<A, B> Protocol for ReceiveChannel<A, B>
where
  A: Protocol,
  B: Protocol,
{
  type ConsumerEndpoint =
    (SenderOnce<A::ConsumerEndpoint>, B::ConsumerEndpoint);
  type ProviderEndpoint =
    (ReceiverOnce<A::ConsumerEndpoint>, B::ProviderEndpoint);
}

impl<A, P, Q> RecApp<A> for ReceiveChannel<P, Q>
where
  P: RecApp<A>,
  Q: RecApp<A>,
{
  type Applied = ReceiveChannel<P::Applied, Q::Applied>;
}

impl<A, B, X> SharedRecApp<X> for ReceiveChannel<A, B>
where
  B: SharedRecApp<X>,
{
  type Applied = ReceiveChannel<A, B::Applied>;
}
