use crate::internal::{
  base::{
    channel::{
      ReceiverOnce,
      SenderOnce,
    },
    rec::RecX,
  },
  functional::{
    nat::*,
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
}

pub trait SharedProtocol: Send + 'static {}

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
}

impl Protocol for Z
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();
}

impl<N> Protocol for S<N>
where
  N: Protocol,
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();
}
