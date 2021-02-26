use crate::internal::base::*;

pub struct ReceiveChannel<P, Q>(
  pub(crate) SenderOnce<(ReceiverOnce<P>, SenderOnce<Q>)>,
);

impl<P, Q> Protocol for ReceiveChannel<P, Q>
where
  P : Protocol,
  Q : Protocol,
{
}

impl<A, P, Q> RecApp<A> for ReceiveChannel<P, Q>
where
  P : RecApp<A>,
  Q : RecApp<A>,
{
  type Applied = ReceiveChannel<P::Applied, Q::Applied>;
}

impl<A, B, X> SharedRecApp<X> for ReceiveChannel<A, B>
where
  B : SharedRecApp<X>,
{
  type Applied = ReceiveChannel<A, B::Applied>;
}
