use crate::internal::functional::nat::Nat;

pub struct Empty;

pub trait SealedContext {}

pub trait Context: SealedContext + Send + 'static
{
  type Endpoints: Sized + Send;

  type Length: Nat;
}

pub trait SealedEmptyContext {}

pub trait EmptyContext: SealedEmptyContext + Context
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

pub trait SealedSlot {}

pub trait Slot: SealedSlot + Send + 'static
{
  type Endpoint: Send;
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
