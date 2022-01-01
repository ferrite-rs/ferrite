use crate::internal::{
  base::{
    channel::{
      once_channel,
      ReceiverOnce,
      SenderOnce,
    },
    rec::{
      RecX,
      Release,
    },
  },
  functional::{
    nat::*,
    row::*,
    type_app::{
      TyCon,
      TypeApp,
    },
  },
};

pub trait Protocol: Send + 'static
{
  type ProviderEndpoint: Send + 'static;
  type ConsumerEndpoint: Send + 'static;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint);
}

pub trait SharedProtocol: Send + 'static {}

pub trait ProtocolRow: RowCon
{
  fn create_row_endpoints() -> (
    AppSum<Self, ProviderEndpointF>,
    AppSum<Self, ConsumerEndpointF>,
  );
}

pub struct ProviderEndpointF;

pub struct ConsumerEndpointF;

impl TyCon for ProviderEndpointF {}
impl TyCon for ConsumerEndpointF {}

impl<A: Protocol> TypeApp<A> for ProviderEndpointF
{
  type Applied = A::ProviderEndpoint;
}

impl<A: Protocol> TypeApp<A> for ConsumerEndpointF
{
  type Applied = A::ConsumerEndpoint;
}

impl<C, F> Protocol for RecX<C, F>
where
  C: Send + 'static,
  F: Send + 'static,
{
  type ConsumerEndpoint = ReceiverOnce<RecX<C, F>>;
  type ProviderEndpoint = SenderOnce<RecX<C, F>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    once_channel()
  }
}

impl Protocol for Release
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    ((), ())
  }
}

impl Protocol for Z
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    ((), ())
  }
}

impl<N> Protocol for S<N>
where
  N: Protocol,
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    ((), ())
  }
}
