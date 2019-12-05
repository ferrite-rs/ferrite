use async_std::task;
use async_std::sync::{ Sender, Receiver, channel };
use async_macros::join;

use crate::process::{ SendChannel };
use crate::base::*;
use crate::processes::*;

/*
    Additive Conjunction, Right Rule

              cont :: Δ  ⊢ Q
    =======================================
      send_channel_from (cont) :: P, Δ  ⊢ P ⊗ Q
 */
pub fn send_channel_from
  < S, T, D, P, Q, Lens >
  ( _ : Lens,
    cont: PartialSession < T, Q >
  ) ->
     PartialSession <
      S,
      SendChannel< P, Q >
    >
where
  P : Process + 'static,
  Q : Process + 'static,
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  Lens :
    ProcessLens <
      S, T, D,
      P,
      Inactive
    >
{
  return PartialSession {
    builder: Box::new(move | ins1, sender1 | {
      let (p_chan, ins2) =
        < Lens as
          ProcessLens <
            S, T, D,
            P,
            Inactive
          >
        > :: split_channels (ins1);

      let (sender2, receiver2) = channel(1);
      let (sender3, receiver3) = channel(1);

      let ins3 =
        < Lens as
          ProcessLens <
            S, T, D,
            P,
            Inactive
          >
        > :: merge_channels ((), ins2);

      Box::pin(async {
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
          (cont.builder)(ins3, sender3).await;
        });

        join!(child1, child2, child3).await;
      })
    })
  }
}

/*
  Additive Conjunction, Left Rule

            cont :: P, Q, Δ  ⊢ S
  ==========================================
    receive_channel_from(cont) :: P ⊗ Q, Δ  ⊢ S
 */
pub fn receive_channel_from
  < S, T, D,
    P1, P2, Q,
    SourceLens,
    F
  >
  ( _ : SourceLens,
    cont_builder: F
  ) ->
    PartialSession < S, Q >
where
  P1 : Process + 'static,
  P2 : Process + 'static,
  Q : Process + 'static,
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  T : NextSelector,
  T : Appendable <
        ( P1, () )
      >,
  SourceLens :
    ProcessLens <
      S, T, D,
      SendChannel < P1, P2 >,
      P2
    >,
  F : FnOnce
        ( < T as NextSelector > :: Selector
        ) ->
          PartialSession <
            < T as
              Appendable <
                ( P1, () )
              >
            > :: AppendResult,
            Q
          >
{
  let cont = cont_builder (
    < T as NextSelector > :: make_selector ()
  );

  return PartialSession {
    builder: Box::new( move | ins1, sender1 | {
      let ( pair_chan, ins2 ) =
        < SourceLens as
          ProcessLens <
            S, T, D,
            SendChannel < P1, P2 >,
            P2
          >
        > :: split_channels ( ins1 );

      Box::pin(async move {
        let (p_chan, y_chan) = pair_chan.recv().await.unwrap();

        let ins3 =
          < SourceLens as
            ProcessLens <
              S, T, D,
              SendChannel < P1, P2 >,
              P2
            >
          > :: merge_channels ( y_chan, ins2 );

        let ins4 =
          < T as
            Appendable <
              ( P1, () )
            >
          > :: append_channels (ins3, (p_chan, ()));

        (cont.builder)(ins4, sender1).await;
      })
    })
  };
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
  return  PartialSession {
    builder: Box::new(move |
      ins,
      sender: Sender<(
        Receiver< P::Value >,
        Receiver< Q::Value >
      )>
    | {
      let (ins1, ins2) = < InsP as Appendable<InsQ> >::split_channels(ins);

      let (sender1, receiver1) = channel(1);
      let (sender2, receiver2) = channel(1);

      Box::pin(async move {
        // the first thread spawns immediately

        let child1 = task::spawn(async move {
          (cont1.builder)(ins1, sender1).await;
        });

        // the sender here blocks until the inner channel pairs
        // are received on the other side
        let child2 = task::spawn(async move {
          sender.send((receiver1, receiver2)).await;
        });

        // the second thread is blocked until the first channel is being accessed

        let child3 = task::spawn(async move {
          (cont2.builder)(ins2, sender2).await;
        });

        join!(child1, child2, child3).await;
      })
    })
  }
}

pub fn receive_channel_from_slot
  < S,
    T1, T2, D1, D2,
    P1, P2, Q,
    TargetLens, SourceLens
  >
  (
    _ : SourceLens,
    _ : TargetLens,
    cont:
      PartialSession < T2, Q >
  ) ->
    PartialSession < S, Q >
where
  P1 : Process + 'static,
  P2 : Process + 'static,
  Q : Process + 'static,
  S : Processes + 'static,
  T1 : Processes + 'static,
  T2 : Processes + 'static,
  D1 : Processes + 'static,
  D2 : Processes + 'static,
  SourceLens :
    ProcessLens <
      S, T1, D1,
      SendChannel < P1, P2 >,
      P2
    >,
  TargetLens :
    ProcessLens <
      T1, T2, D2,
      Inactive,
      P1
    >,
{
  return  PartialSession {
    builder: Box::new( move | ins1, sender1 | {
      let ( pair_chan, ins2 ) =
        < SourceLens as
          ProcessLens <
            S, T1, D1,
            SendChannel < P1, P2 >,
            P2
          >
        > :: split_channels ( ins1 );

      Box::pin(async move {
        let (p_chan, y_chan) = pair_chan.recv().await.unwrap();

        let ins3 =
          < SourceLens as
            ProcessLens <
              S, T1, D1,
              SendChannel < P1, P2 >,
              P2
            >
          > :: merge_channels ( y_chan, ins2 );

        let ((), ins4) =
          < TargetLens as
            ProcessLens <
              T1, T2, D2,
              Inactive,
              P1
            >
          > :: split_channels ( ins3 );

        let ins5 =
          < TargetLens as
            ProcessLens <
              T1, T2, D2,
              Inactive,
              P1
            >
          > :: merge_channels ( p_chan, ins4 );

        (cont.builder)(ins5, sender1).await;
      })
    })
  };
}
