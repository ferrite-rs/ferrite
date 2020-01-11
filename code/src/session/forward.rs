
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
  < I, P, Lens >
  (_ : Lens)
  ->
    PartialSession <
      I,
      P
    >
where
  P : Process + 'static,
  I : Processes + 'static,
  Lens :: Target : EmptyList,
  Lens :
    ProcessLens <
      I,
      P,
      Inactive
    >
{
  create_partial_session (
    async move | ins, sender | {
      let (receiver, _) =
        < Lens as
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
