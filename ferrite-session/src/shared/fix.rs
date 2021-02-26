use crate::{
  base::Protocol,
  functional::{
    nat::*,
    row::*,
  },
  protocol::{
    ReceiveChannel,
    ReceiveValue,
    SendChannel,
    SendValue,
    *,
  },
};

pub trait SharedRecApp<X>
{
  type Applied;
}

impl<X> SharedRecApp<X> for Z
{
  type Applied = X;
}

impl<T, A, X> SharedRecApp<X> for SendValue<T, A>
where
  T : Send + 'static,
  A : SharedRecApp<X>,
{
  type Applied = SendValue<T, A::Applied>;
}

impl<T, A, X> SharedRecApp<X> for ReceiveValue<T, A>
where
  T : Send + 'static,
  A : SharedRecApp<X>,
{
  type Applied = ReceiveValue<T, A::Applied>;
}

impl<R> SharedRecApp<R> for ()
{
  type Applied = ();
}

impl<P, Q, R> SharedRecApp<R> for (P, Q)
where
  P : SharedRecApp<R>,
  Q : SharedRecApp<R>,
{
  type Applied = (P::Applied, Q::Applied);
}

impl<P, Q, R> SharedRecApp<R> for SendChannel<P, Q>
where
  P : Protocol,
  Q : SharedRecApp<R>,
{
  type Applied = SendChannel<P, Q::Applied>;
}

impl<A, B, X> SharedRecApp<X> for ReceiveChannel<A, B>
where
  B : SharedRecApp<X>,
{
  type Applied = ReceiveChannel<A, B::Applied>;
}

impl<Row, A> SharedRecApp<A> for InternalChoice<Row>
where
  Row : RowApp<ReceiverF>,
  Row : SharedRecApp<A>,
  <Row as SharedRecApp<A>>::Applied : RowApp<ReceiverF>,
{
  type Applied = InternalChoice<<Row as SharedRecApp<A>>::Applied>;
}

impl<Row, A> SharedRecApp<A> for ExternalChoice<Row>
where
  Row : SharedRecApp<A>,
  Row : RowApp<()>,
  Row : RowApp<ReceiverF>,
  <Row as SharedRecApp<A>>::Applied : RowApp<()>,
  <Row as SharedRecApp<A>>::Applied : RowApp<ReceiverF>,
{
  type Applied = ExternalChoice<<Row as SharedRecApp<A>>::Applied>;
}
