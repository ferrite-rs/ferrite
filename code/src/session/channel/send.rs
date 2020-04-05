use async_std::task;
use async_macros::join;
use async_std::sync::{
  channel
};

use crate::process::{ SendChannel };

use crate::base::{
  Protocol,
  Empty,
  Context,
  AppendContext,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
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
  < I, P, Q, N >
  ( _ : N,
    cont:
      PartialSession <
        N :: Target,
        Q
      >
  ) ->
    PartialSession <
      I,
      SendChannel< P, Q >
    >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  N :
    ContextLens <
      I,
      P,
      Empty
    >
{
  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (p_chan, ctx2) =
        < N as
          ContextLens <
            I, P, Empty
          >
        > :: extract_source (ctx1);

      let (sender2, receiver2) = channel(1);
      let (sender3, receiver3) = channel(1);

      let ctx3 =
        < N as
          ContextLens <
            I, P, Empty
          >
        > :: insert_target ((), ctx2);

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
        unsafe_run_session
          ( cont, ctx3, sender3
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
    N,
    F
  >
  ( _ : N,
    cont_builder: F
  ) ->
    PartialSession < I, Q >
where
  P1 : Protocol,
  P2 : Protocol,
  Q : Protocol,
  I : Context,
  N :: Target :
    NextSelector,
  N :: Target :
    AppendContext <
        ( P1, () )
      >,
  N :
    ContextLens <
      I,
      SendChannel < P1, P2 >,
      P2
    >,
  F : FnOnce
        ( < N :: Target
            as NextSelector
          > :: Selector
        ) ->
          PartialSession <
            < N :: Target
              as AppendContext <
                ( P1, () )
              >
            > :: Appended,
            Q
          >
{
  let cont = cont_builder (
    < N :: Target
      as NextSelector
    > :: make_selector ()
  );

  unsafe_create_session (
    async move | ctx1, sender1 | {
      let ( pair_chan, ctx2 ) =
        < N as
          ContextLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: extract_source ( ctx1 );

      let (p_chan, y_chan) = pair_chan.recv().await.unwrap();

      let ctx3 =
        < N as
          ContextLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: insert_target ( y_chan, ctx2 );

      let ctx4 =
        < N :: Target as
          AppendContext <
            ( P1, () )
          >
        > :: append_context (ctx3, (p_chan, ()));

        unsafe_run_session
          ( cont, ctx4, sender1
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
pub fn fork <P, Q, CP, CQ>
  (
    cont1:  PartialSession <CP, P>,
    cont2:  PartialSession <CQ, Q>
  ) ->
     PartialSession <
      < CP as AppendContext<CQ> >::Appended,
      SendChannel<P, Q>
    >
where
  P: Protocol,
  Q: Protocol,
  CP: Context,
  CQ: Context,
  CP: AppendContext<CQ>,
  P: 'static,
  Q: 'static,
  CP: 'static,
  CQ: 'static
{
  unsafe_create_session (
    async move | ctx, sender | {
      let (ctx1, ctx2) = CP :: split_context(ctx);

      let (sender1, receiver1) = channel(1);
      let (sender2, receiver2) = channel(1);

      // the first thread spawns immediately

      let child1 = task::spawn(async move {
        unsafe_run_session
          ( cont1, ctx1, sender1
          ).await;
      });

      // the sender here blocks until the inner channel pairs
      // are received on the other side
      let child2 = task::spawn(async move {
        sender.send((receiver1, receiver2)).await;
      });

      // the second thread is blocked until the first channel is being accessed

      let child3 = task::spawn(async move {
        unsafe_run_session
          ( cont2, ctx2, sender2
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
  P1 : Protocol,
  P2 : Protocol,
  Q : Protocol,
  I : Context,
  SourceLens :
    ContextLens <
      I,
      SendChannel < P1, P2 >,
      P2
    >,
  TargetLens :
    ContextLens <
      SourceLens :: Target,
      Empty,
      P1
    >,
{
  unsafe_create_session (
    async move | ctx1, sender1 | {
      let ( pair_chan, ctx2 ) =
        < SourceLens as
          ContextLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: extract_source ( ctx1 );

      let (p_chan, y_chan) =
        pair_chan.recv().await.unwrap();

      let ctx3 =
        < SourceLens as
          ContextLens <
            I,
            SendChannel < P1, P2 >,
            P2
          >
        > :: insert_target ( y_chan, ctx2 );

      let ((), ctx4) =
        < TargetLens as
          ContextLens <
            SourceLens :: Target,
            Empty,
            P1
          >
        > :: extract_source ( ctx3 );

      let ctx5 =
        < TargetLens as
          ContextLens <
            SourceLens :: Target,
            Empty,
            P1
          >
        > :: insert_target ( p_chan, ctx4 );

        unsafe_run_session
          ( cont, ctx5, sender1
          ).await;
    })
}
