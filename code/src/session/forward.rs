
use crate::base::{
  Protocol,
  Empty,
  Context,
  EmptyContext,
  ContextLens,
  PartialSession,
  unsafe_create_session,
};

pub fn forward
  < I, P, N >
  (_ : N)
  ->
    PartialSession <
      I,
      P
    >
where
  P : Protocol,
  I : Context,
  N :: Target : EmptyContext,
  N :
    ContextLens <
      I,
      P,
      Empty
    >
{
  unsafe_create_session (
    async move | ins, sender | {
      let (receiver, _) = N :: split_channels ( ins );

      let val = receiver.recv().await.unwrap();
      sender.send( val ).await;
    })
}
