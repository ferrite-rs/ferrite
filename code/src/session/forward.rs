
use crate::base::*;

pub fn forward
  < S, T, D, P, Lens >
  (_ : Lens)
  ->
    PartialSession <
      S,
      P
    >
where
  P : Process + 'static,
  S : Processes + 'static,
  T : Processes + EmptyList + 'static,
  D : Processes + EmptyList + 'static,
  Lens :
    ProcessLens <
      S, T, D,
      P,
      Inactive
    >
{
  create_partial_session ( 
    async move | ins, sender | {
      let (receiver, _) =
        < Lens as
          ProcessLens <
            S, T, D,
            P,
            Inactive
          >
        > :: split_channels ( ins );

      let val = receiver.recv().await.unwrap();
      sender.send( val ).await;
    })
}
