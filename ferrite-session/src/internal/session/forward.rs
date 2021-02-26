use crate::internal::base::{
  unsafe_create_session,
  Context,
  ContextLens,
  Empty,
  EmptyContext,
  PartialSession,
  Protocol,
};

pub fn forward<N, C, A>(_ : N) -> PartialSession<C, A>
where
  A : Protocol,
  C : Context,
  N::Target : EmptyContext,
  N : ContextLens<C, A, Empty>,
{
  unsafe_create_session(move |ctx, sender| async move {
    let (receiver, _) = N::extract_source(ctx);

    let val = receiver.recv().await.unwrap();

    sender.send(val).unwrap();
  })
}
