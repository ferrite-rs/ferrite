use crate::internal::{
  base::protocol::{
    ClientEndpoint,
    Protocol,
  },
  functional::nat::{
    Nat,
    S,
    Z,
  },
};

pub trait Context: Send + 'static
{
  type Endpoints: Sized + Send;

  type Length: Nat;
}

pub trait EmptyContext: Context
{
  fn empty_values() -> <Self as Context>::Endpoints;
}

pub trait AppendContext<R>: Context
where
  R: Context,
{
  type Appended: Context;

  fn append_context(
    channels1: <Self as Context>::Endpoints,
    channels2: <R as Context>::Endpoints,
  ) -> <Self::Appended as Context>::Endpoints;

  fn split_context(
    channels: <Self::Appended as Context>::Endpoints
  ) -> (<Self as Context>::Endpoints, <R as Context>::Endpoints);
}

pub trait Slot: Send + 'static
{
  type Endpoint: Send;
}

impl<A> Slot for A
where
  A: Protocol,
{
  type Endpoint = ClientEndpoint<A>;
}

pub struct Empty;

impl Slot for Empty
{
  type Endpoint = ();
}

pub trait ContextLens<C, A1, A2>: Send + 'static
where
  C: Context,
  A1: Slot,
  A2: Slot,
{
  type Deleted: Context;

  type Target: Context;

  fn extract_source(
    channels: C::Endpoints
  ) -> (A1::Endpoint, <Self::Deleted as Context>::Endpoints);

  fn insert_target(
    receiver: A2::Endpoint,
    channels: <Self::Deleted as Context>::Endpoints,
  ) -> <Self::Target as Context>::Endpoints;
}

impl<C, A1, A2> ContextLens<(A1, C), A1, A2> for Z
where
  A1: Slot,
  A2: Slot,
  C: Context,
{
  type Deleted = C;
  type Target = (A2, C);

  fn extract_source(
    ctx: (A1::Endpoint, C::Endpoints)
  ) -> (A1::Endpoint, C::Endpoints)
  {
    ctx
  }

  fn insert_target(
    p: A2::Endpoint,
    r: C::Endpoints,
  ) -> (A2::Endpoint, C::Endpoints)
  {
    (p, r)
  }
}

impl<B, A1, A2, C, N> ContextLens<(B, C), A1, A2> for S<N>
where
  B: Slot,
  A1: Slot,
  A2: Slot,
  C: Context,
  N: ContextLens<C, A1, A2>,
{
  type Deleted = (B, N::Deleted);
  type Target = (B, N::Target);

  fn extract_source(
    (p, r1): (B::Endpoint, C::Endpoints)
  ) -> (
    A1::Endpoint,
    (B::Endpoint, <N::Deleted as Context>::Endpoints),
  )
  {
    let (q, r2) = N::extract_source(r1);

    (q, (p, r2))
  }

  fn insert_target(
    q: A2::Endpoint,
    (p, r1): (B::Endpoint, <N::Deleted as Context>::Endpoints),
  ) -> (B::Endpoint, <N::Target as Context>::Endpoints)
  {
    let r2 = N::insert_target(q, r1);

    (p, r2)
  }
}

impl Context for ()
{
  type Endpoints = ();
  type Length = Z;
}

impl EmptyContext for ()
{
  fn empty_values() {}
}

impl<R> EmptyContext for (Empty, R)
where
  R: EmptyContext,
{
  fn empty_values() -> ((), R::Endpoints)
  {
    ((), R::empty_values())
  }
}

impl<P, R> Context for (P, R)
where
  P: Slot,
  R: Context,
{
  type Endpoints = (P::Endpoint, R::Endpoints);
  type Length = S<R::Length>;
}

impl<R: Context> AppendContext<R> for ()
{
  type Appended = R;

  fn append_context(
    _: (),
    r: <R as Context>::Endpoints,
  ) -> <R as Context>::Endpoints
  {
    r
  }

  fn split_context(
    r: <R as Context>::Endpoints
  ) -> ((), <R as Context>::Endpoints)
  {
    ((), r)
  }
}

impl<P, R, S> AppendContext<S> for (P, R)
where
  P: Slot,
  R: Context,
  S: Context,
  R: AppendContext<S>,
{
  type Appended = (P, <R as AppendContext<S>>::Appended);

  fn append_context(
    (p, r): (P::Endpoint, R::Endpoints),
    s: <S as Context>::Endpoints,
  ) -> (<P as Slot>::Endpoint, <R::Appended as Context>::Endpoints)
  {
    (p, <R as AppendContext<S>>::append_context(r, s))
  }

  fn split_context(
    (p, r): (P::Endpoint, <R::Appended as Context>::Endpoints)
  ) -> (<(P, R) as Context>::Endpoints, <S as Context>::Endpoints)
  {
    let (r2, s) = R::split_context(r);

    ((p, r2), s)
  }
}
