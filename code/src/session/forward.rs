
use crate::base::{
  Protocol,
  Empty,
  Context,
  EmptyContext,
  ContextLens,
  PartialSession,
  create_partial_session,
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
  P : Protocol + 'static,
  I : Context + 'static,
  N :: Target : EmptyContext,
  N :
    ContextLens <
      I,
      P,
      Empty
    >
{
  create_partial_session (
    async move | ins, sender | {
      let (receiver, _) =
        < N as
          ContextLens <
            I,
            P,
            Empty
          >
        > :: split_channels ( ins );

      let val = receiver.recv().await.unwrap();
      sender.send( val ).await;
    })
}
