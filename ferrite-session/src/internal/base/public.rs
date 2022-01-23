#[doc(inline)]
pub use super::{
  AppendContext,
  Context,
  Empty,
  EmptyContext,
  PartialSession,
  Protocol,
  Rec,
  RecX,
  Release,
  Session,
  SharedChannel,
  SharedProtocol,
  SharedSession,
  Slot,
};

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
