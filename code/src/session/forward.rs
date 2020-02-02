
use crate::base::{
  Process,
  Inactive,
  Processes,
  EmptyList,
  ProcessLens,
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
  P : Process + 'static,
  I : Processes + 'static,
  N :: Target : EmptyList,
  N :
    ProcessLens <
      I,
      P,
      Inactive
    >
{
  create_partial_session (
    async move | ins, sender | {
      let (receiver, _) =
        < N as
          ProcessLens <
            I,
            P,
            Inactive
          >
        > :: split_channels ( ins );

      let val = receiver.recv().await.unwrap();
      sender.send( val ).await;
    })
}
