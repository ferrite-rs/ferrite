use crate::base::nat::{ Nat };

/// A list of processes for input. It has multiple implementations including
/// [crate::base::Context].
pub trait Context : 'static {
  type Values : Sized + Send;

  type Length : Nat;
}

/// An ordered linked list of processes.
pub trait EmptyContext : Context {
  fn empty_values () ->
    < Self as Context > :: Values;
}

pub trait
  AppendContext < R > : Context
where
  R : Context
{
  type AppendResult : Context;

  fn append_channels(
    channels1: <Self as Context>::Values,
    channels2: <R as Context>::Values
  ) ->
    <Self::AppendResult as Context>::Values;

  fn split_channels(
    channels: <Self::AppendResult as Context>::Values
  ) -> (
    <Self as Context>::Values,
    <R as Context>::Values
  );
}

pub trait Reversible : Context {
  type Reversed : Context;

  fn reverse_channels(
    channels: <Self as Context>::Values,
  ) ->
    <Self::Reversed as Context>::Values;

  fn unreverse_channels(
    channels: <Self::Reversed as Context>::Values,
  ) ->
    Self::Values;
}
