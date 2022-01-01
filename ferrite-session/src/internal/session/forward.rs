use crate::internal::base::{
  unsafe_create_session,
  Context,
  ContextLens,
  Empty,
  EmptyContext,
  PartialSession,
  Protocol,
};

pub fn forward<N, C, A>(_: N) -> PartialSession<C, A>
where
  A: Protocol,
  C: Context,
  N::Target: EmptyContext,
  N: ContextLens<C, A, Empty>,
{
  unsafe_create_session::<C, A, _, _>(move |ctx, provider_end| async move {
    let (consumer_end, _) = N::extract_source(ctx);
    A::forward(consumer_end.get_applied(), provider_end).await;
  })
}
