use async_std::task;
use async_macros::join;
use async_std::sync::{
  Sender,
  Receiver,
  channel
};

use crate::process::{ SendChannel };

use crate::base::{
  Process,
  Inactive,
  Processes,
  Appendable,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

use crate::processes::{
  NextSelector
};

/*
    Additive Conjunction, Right Rule

              cont :: Δ  ⊢ Q
    =======================================
      send_channel_from (cont) :: P, Δ  ⊢ P ⊗ Q
 */
pub fn send_channel_from
  < I, P, Q, Lens >
  ( _ : Lens,
    cont:
      PartialSession <
        Lens :: Target,
        Q
      >
  ) ->
    PartialSession <
      I,
      SendChannel< P, Q >
    >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      P,
      Inactive
    >
{
  create_partial_session (
    async move | ins1, sender1 | {
      let (p_chan, ins2) =
        < Lens as
          ProcessLens <
            I, P, Inactive
          >
        > :: split_channels (ins1);

      let (sender2, receiver2) = channel(1);
      let (sender3, receiver3) = channel(1);

      let ins3 =
        < Lens as
          ProcessLens <
            I, P, Inactive
          >
        > :: merge_channels ((), ins2);

      let child1 = task::spawn(async move {
        // receive the input x from the input channel
        let p = p_chan.recv().await.unwrap();
        sender2.send(p).await;
      });

      let child2 = task::spawn(async move {
        // blocks until the channel pairs are sent
        sender1.send((receiver2, receiver3)).await;
      });

      let child3 = task::spawn(async {
        // the continuation Q only starts after that
        run_partial_session
          ( cont, ins3, sender3
          ).await;
      });

      join!(child1, child2, child3).await;
  })
}

/*
  Additive Conjunction, Left Rule

            cont :: P, Q, Δ  ⊢ S
  ==========================================
    receive_channel_from(cont) :: P ⊗ Q, Δ  ⊢ S
 */
pub fn receive_channel_from
  < I, P1, P2, Q,
    Lens,
    F
  >
  ( _ : Lens,
    cont_builder: F
  ) ->
    PartialSession < I, Q >
where
  P1 : Process + 'static,
  P2 : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  Lens :: Target :
    NextSelector,
  Lens :: Target :
    Appendable <
        ( P1, () )
      >,
  Lens :
    ProcessLens <
      I,
      SendChannel < P1, P2 >,
      P2
    >,
  F : FnOnce
        ( < Lens :: Target
            as NextSelector
          > :: Selector
        ) ->
          PartialSession <
            < Lens :: Target
              as Appendable <
                ( P1, () )
              >
            > :: AppendResult,
            Q
          >
{
  let cont = cont_builder (
    < Lens :: Target
      as NextSelector
    > :: make_selector ()
  );

  create_partial_session (
    async move | ins1, sender1 | {
      let ( pair_chan, ins2 ) =
        < Lens as
          ProcessLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: split_channels ( ins1 );

      let (p_chan, y_chan) = pair_chan.recv().await.unwrap();

      let ins3 =
        < Lens as
          ProcessLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: merge_channels ( y_chan, ins2 );

      let ins4 =
        < Lens :: Target as
          Appendable <
            ( P1, () )
          >
        > :: append_channels (ins3, (p_chan, ()));

        run_partial_session
          ( cont, ins4, sender1
          ).await;
    })
}

/*
    Multiplicative Conjunction, Alternative Parallel Version


       cont1 :: Δ ⊢ P    cont2 :: Δ'  ⊢ Q
    ========================================
      fork(cont1, cont2) :: Δ, Δ' ⊢ P ⊗ Q

    Takes in two session builders and return a new session builder
    with its inputs combined and outputs a parallel processes
 */
pub fn fork <P, Q, InsP, InsQ>
  (
    cont1:  PartialSession <InsP, P>,
    cont2:  PartialSession <InsQ, Q>
  ) ->
     PartialSession <
      < InsP as Appendable<InsQ> >::AppendResult,
      SendChannel<P, Q>
    >
where
  P: Process,
  Q: Process,
  InsP: Processes,
  InsQ: Processes,
  InsP: Appendable<InsQ>,
  P: 'static,
  Q: 'static,
  InsP: 'static,
  InsQ: 'static
{
  create_partial_session (
    async move |
      ins,
      sender: Sender<(
        Receiver< P::Value >,
        Receiver< Q::Value >
      )>
    | {
      let (ins1, ins2) = < InsP as Appendable<InsQ> >::split_channels(ins);

      let (sender1, receiver1) = channel(1);
      let (sender2, receiver2) = channel(1);

      // the first thread spawns immediately

      let child1 = task::spawn(async move {
        run_partial_session
          ( cont1, ins1, sender1
          ).await;
      });

      // the sender here blocks until the inner channel pairs
      // are received on the other side
      let child2 = task::spawn(async move {
        sender.send((receiver1, receiver2)).await;
      });

      // the second thread is blocked until the first channel is being accessed

      let child3 = task::spawn(async move {
        run_partial_session
          ( cont2, ins2, sender2
          ).await;
      });

      join!(child1, child2, child3).await;
    })
}

pub fn receive_channel_from_slot
  < I, P1, P2, Q,
    TargetLens, SourceLens
  >
  (
    _ : SourceLens,
    _ : TargetLens,
    cont:
      PartialSession <
        TargetLens :: Target,
        Q
      >
  ) ->
    PartialSession < I, Q >
where
  P1 : Process + 'static,
  P2 : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  SourceLens :
    ProcessLens <
      I,
      SendChannel < P1, P2 >,
      P2
    >,
  TargetLens :
    ProcessLens <
      SourceLens :: Target,
      Inactive,
      P1
    >,
{
  create_partial_session (
    async move | ins1, sender1 | {
      let ( pair_chan, ins2 ) =
        < SourceLens as
          ProcessLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: split_channels ( ins1 );

      let (p_chan, y_chan) =
        pair_chan.recv().await.unwrap();

      let ins3 =
        < SourceLens as
          ProcessLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: merge_channels ( y_chan, ins2 );

      let ((), ins4) =
        < TargetLens as
          ProcessLens <
            SourceLens :: Target,
            Inactive,
            P1
          >
        > :: split_channels ( ins3 );

      let ins5 =
        < TargetLens as
          ProcessLens <
            SourceLens :: Target,
            Inactive,
            P1
          >
        > :: merge_channels ( p_chan, ins4 );

        run_partial_session
          ( cont, ins5, sender1
          ).await;
    })
}
