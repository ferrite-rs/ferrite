#[doc(inline)]
pub use super::{
  Empty,
  PartialSession,
  Rec,
  RecX,
  Release,
  Session,
  SharedChannel,
  SharedSession,
};

pub trait Protocol: super::Protocol {}

impl<A> Protocol for A where A: super::Protocol {}

pub trait SharedProtocol: super::SharedProtocol {}

impl<A> SharedProtocol for A where A: super::SharedProtocol {}

pub trait Context: super::Context {}

impl<C> Context for C where C: super::Context {}

pub trait EmptyContext: super::EmptyContext {}

pub trait Slot: super::Slot {}

impl<C> Slot for C where C: super::Slot {}

impl<C> EmptyContext for C where C: super::EmptyContext {}

pub trait AppendContext<C>: super::AppendContext<C>
where
  C: Context,
{
}

impl<C1, C2> AppendContext<C2> for C1
where
  C2: Context,
  C1: super::AppendContext<C2>,
{
}

pub trait ContextLens<C, A1, A2>: super::ContextLens<C, A1, A2>
where
  C: Context,
  A1: Slot,
  A2: Slot,
{
}

impl<N, C, A1, A2> ContextLens<C, A1, A2> for N
where
  C: Context,
  A1: Slot,
  A2: Slot,
  N: super::ContextLens<C, A1, A2>,
{
}

pub trait RecApp<A>: super::RecApp<A> {}

impl<A, X> RecApp<A> for X where X: super::RecApp<A> {}

pub trait SharedRecApp<X>: super::SharedRecApp<X> {}

impl<X, S> SharedRecApp<X> for S where S: super::SharedRecApp<X> {}

pub trait HasRecApp<F, A>: super::HasRecApp<F, A> {}

impl<F, A, X> HasRecApp<F, A> for X where X: super::HasRecApp<F, A> {}

pub trait ForwardChannel: super::ForwardChannel {}

impl<A> ForwardChannel for A where A: super::ForwardChannel {}
