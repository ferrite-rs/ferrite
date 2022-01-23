use core::{
  future::Future,
  pin::Pin,
};

use super::{
  traits::{
    HasRecApp,
    HasRecEndpoint,
    RecApp,
    SharedRecApp,
  },
  types::{
    RecEndpoint,
    RecRow,
    RecX,
    Release,
    SharedRecRow,
  },
};
use crate::internal::{
  base::{
    channel::{
      once_channel,
      ReceiverOnce,
      SenderOnce,
    },
    protocol::{
      Protocol,
      SealedProtocol,
    },
  },
  functional::*,
};

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

impl<C, F> RecApp<C> for RecX<(), F>
where
  C: Send,
  F: RecApp<(RecX<C, F>, C)>,
{
  type Applied = RecX<C, F>;
}

impl<C, A> RecApp<(A, C)> for Z
where
  A: Send,
  C: Send,
{
  type Applied = A;
}

impl<C, A, N> RecApp<(A, C)> for S<N>
where
  N: Send,
  C: Send,
  A: Send,
  N: RecApp<C>,
{
  type Applied = N::Applied;
}

impl<A> RecApp<A> for ()
{
  type Applied = ();
}

impl<A, X, Y> RecApp<A> for (X, Y)
where
  X: RecApp<A>,
  Y: RecApp<A>,
{
  type Applied = (X::Applied, Y::Applied);
}

impl<X> SharedRecApp<X> for Release
{
  type Applied = X;
}

impl<R> SharedRecApp<R> for ()
{
  type Applied = ();
}

impl<P, Q, R> SharedRecApp<R> for (P, Q)
where
  P: SharedRecApp<R>,
  Q: SharedRecApp<R>,
{
  type Applied = (P::Applied, Q::Applied);
}

impl<X, F> SharedRecApp<X> for RecX<(), F>
where
  F: SharedRecApp<X>,
{
  type Applied = RecX<(), F::Applied>;
}

impl<F, C, E> HasRecEndpoint<F, C> for E
where
  E: Send + 'static,
  F: RecApp<C>,
  F::Applied: Protocol<ClientEndpoint = E>,
{
  fn get_applied(self: Box<Self>) -> Box<Self>
  {
    self
  }
}

impl<C, F> SealedProtocol for RecX<C, F> {}

impl<C, F> Protocol for RecX<C, F>
where
  C: Send + 'static,
  F: Protocol,
  F: RecApp<(RecX<C, F>, C)>,
{
  type ClientEndpoint = ReceiverOnce<RecEndpoint<F, (RecX<C, F>, C)>>;
  type ProviderEndpoint = SenderOnce<RecEndpoint<F, (RecX<C, F>, C)>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    once_channel()
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let endpoint = client_end.recv().await.unwrap();
      provider_end.send(endpoint).unwrap();
    })
  }
}

impl SealedProtocol for Release {}

impl Protocol for Release
{
  type ClientEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    ((), ())
  }

  fn forward(
    _client_end: Self::ClientEndpoint,
    _provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}

impl<R, Row1, Row2, Row3> ToRow for RecRow<R, Row1>
where
  R: Send,
  Row1: ToRow<Row = Row2>,
  Row2: RecApp<R, Applied = Row3>,
  Row3: RowCon,
{
  type Row = Row3;
}

impl<R, Row1, Row2, Row3> ToRow for SharedRecRow<R, Row1>
where
  R: Send,
  Row1: ToRow<Row = Row2>,
  Row2: SharedRecApp<R, Applied = Row3>,
  Row3: RowCon,
{
  type Row = Row3;
}
