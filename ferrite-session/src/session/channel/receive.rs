use async_macros::join;
use tokio::task;

use crate::{
  base::*,
  functional::*,
  protocol::ReceiveChannel,
  session::{forward::forward, include::include_session},
};

/*
   Implication, Right Rule

         cont :: Δ, P  ⊢ Q
   ====================================
     receive_channel(cont) :: Δ  ⊢ P ⊸ Q
*/

pub fn receive_channel<C, A, B>(
  cont : impl FnOnce(C::Length) -> PartialSession<C::Appended, B>
) -> PartialSession<C, ReceiveChannel<A, B>>
where
  A : Protocol,
  B : Protocol,
  C : Context,
  C : AppendContext<(A, ())>,
{

  let cont2 = cont(C::Length::nat());

  unsafe_create_session(move |ctx1, sender| async move {

    let (sender1, receiver1) = once_channel();

    sender.send(ReceiveChannel(sender1)).unwrap();

    let (receiver2, sender2) = receiver1.recv().await.unwrap();

    let ctx2 = C::append_context(ctx1, (receiver2, ()));

    unsafe_run_session(cont2, ctx2, sender2).await;
  })
}

pub fn receive_channel_slot<I, P, Q, N>(
  _ : N,
  cont : PartialSession<N::Target, Q>,
) -> PartialSession<I, ReceiveChannel<P, Q>>
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  N : ContextLens<I, Empty, P>,
{

  unsafe_create_session(move |ctx1, sender| async move {

    let ((), ctx2) = N::extract_source(ctx1);

    let (sender1, receiver1) = once_channel();

    let child1 = task::spawn(async move {

      sender.send(ReceiveChannel(sender1)).unwrap();
    });

    let child2 = task::spawn(async move {

      let (receiver2, sender2) = receiver1.recv().await.unwrap();

      let ctx3 =
        <N as ContextLens<I, Empty, P>>::insert_target(receiver2, ctx2);

      unsafe_run_session(cont, ctx3, sender2).await;
    });

    let _ = join!(child1, child2).await;
  })
}

/*
   Implication, Left Rule

               cont :: Q, Δ ⊢ S
   ========================================
     send_channel_to(cont) :: P, P ⊸ Q, Δ ⊢ S
*/

pub fn send_channel_to<N1, N2, C, A1, A2, B>(
  _ : N1,
  _ : N2,
  cont : PartialSession<N1::Target, B>,
) -> PartialSession<C, B>
where
  C : Context,
  A1 : Protocol,
  A2 : Protocol,
  B : Protocol,
  N2 : ContextLens<C, A1, Empty>,
  N1 : ContextLens<N2::Target, ReceiveChannel<A1, A2>, A2>,
{

  unsafe_create_session(move |ctx1, sender1| async move {

    let (receiver1, ctx2) = N2::extract_source(ctx1);

    let ctx3 = N2::insert_target((), ctx2);

    let (receiver2, ctx4) = N1::extract_source(ctx3);

    let ReceiveChannel(sender2) = receiver2.recv().await.unwrap();

    let (sender3, receiver3) = once_channel();

    let child1 = task::spawn(async move {

      sender2.send((receiver1, sender3)).unwrap();
    });

    let ctx5 = N1::insert_target(receiver3, ctx4);

    let child2 = task::spawn(async move {

      unsafe_run_session(cont, ctx5, sender1).await;
    });

    let _ = join!(child1, child2).await;
  })
}

/*
   Implication, Application

     p1 :: · ⊢ P ⊸ Q       p2 :: · ⊢ P
   ========================================
       apply_channel(p1, p2) :: · ⊢ Q
*/

pub fn apply_channel<A, B>(
  f : Session<ReceiveChannel<A, B>>,
  a : Session<A>,
) -> Session<B>
where
  A : Protocol,
  B : Protocol,
{

  include_session(f, move |c1| {

    include_session(a, move |c2| send_channel_to(c1, c2, forward(c1)))
  })
}
