pub trait Protocol: crate::base::Protocol
{
}

impl<A> Protocol for A where A : crate::base::Protocol {}

pub trait Context: crate::base::Context
{
}

impl<C> Context for C where C : crate::base::Context {}

pub trait EmptyContext: crate::base::EmptyContext
{
}

pub trait Slot: crate::base::Slot
{
}

impl<C> Slot for C where C : crate::base::Slot {}

impl<C> EmptyContext for C where C : crate::base::EmptyContext {}

pub trait AppendContext<C>: crate::base::AppendContext<C>
where
  C : Context,
{
}

impl<C1, C2> AppendContext<C2> for C1
where
  C2 : Context,
  C1 : crate::base::AppendContext<C2>,
{
}

pub trait ContextLens<C, A1, A2>: crate::base::ContextLens<C, A1, A2>
where
  C : Context,
  A1 : Slot,
  A2 : Slot,
{
}

impl<N, C, A1, A2> ContextLens<C, A1, A2> for N
where
  C : Context,
  A1 : Slot,
  A2 : Slot,
  N : crate::base::ContextLens<C, A1, A2>,
{
}

pub trait RecApp<A>: crate::base::RecApp<A>
{
}

impl<A, X> RecApp<A> for X where X : crate::base::RecApp<A> {}

pub trait HasRecApp<F, A>: crate::base::HasRecApp<F, A>
{
}

impl<F, A, X> HasRecApp<F, A> for X where X : crate::base::HasRecApp<F, A> {}

pub trait ForwardChannel: crate::base::ForwardChannel
{
}

impl<A> ForwardChannel for A where A : crate::base::ForwardChannel {}
