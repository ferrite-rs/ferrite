use crate::base::nat::{ Nat };

/// A list of context for input. It has multiple implementations including
/// [crate::base::Context].
pub trait Context : Send + 'static {
  type Endpoints : Sized + Send;

  type Length : Nat;
}

/// An ordered linked list of context.
pub trait EmptyContext : Context {
  fn empty_values () ->
    < Self as Context > :: Endpoints;
}

pub trait
  AppendContext < R > : Context
where
  R : Context
{
  type Appended : Context;

  fn append_context (
    channels1: <Self as Context>::Endpoints,
    channels2: <R as Context>::Endpoints
  ) ->
    <Self::Appended as Context>::Endpoints;

  fn split_context (
    channels: <Self::Appended as Context>::Endpoints
  ) -> (
    <Self as Context>::Endpoints,
    <R as Context>::Endpoints
  );
}

pub trait Reversible : Context {
  type Reversed : Context;

  fn reverse_channels(
    channels: <Self as Context>::Endpoints,
  ) ->
    <Self::Reversed as Context>::Endpoints;

  fn unreverse_channels(
    channels: <Self::Reversed as Context>::Endpoints,
  ) ->
    Self::Endpoints;
}
