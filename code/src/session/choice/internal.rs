use async_std::task;
use async_macros::join;
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{
  Either,
  InternalChoice,
};

use crate::base::{
  PartialSession,
  Protocol,
  Context,
  ContextLens,
  unsafe_run_session,
  unsafe_create_session,
};

/*
  Additive Disjuction / Internal Choice

  Right Rule (Session)

            cont :: Δ ⊢ P
  =================================
    offer_left(cont) :: Δ ⊢ P ⊕ Q

  offerLeft
    :: forall ctx p q
       ( Protocol p
       , Protocol q
       , Context ctx
       )
    =>  PartialSession ctx p
    ->  PartialSession ctx (InternalChoice p q)
 */
pub fn offer_left
  < I, P, Q >
  ( cont:  PartialSession < I, P >
  ) ->
    PartialSession < I,
      InternalChoice < P, Q >
    >
where
  P : Protocol,
  Q : Protocol,
  I : Context
{
  unsafe_create_session (
    async move |
      ctx,
      sender: Sender<
        Either<
          Receiver<P::Payload>,
          Receiver<Q::Payload> > >
    | {
      let (in_sender, in_receiver) = channel(1);

      let child1 = task::spawn(async {
        unsafe_run_session
          ( cont, ctx, in_sender
          ).await;
      });

      let child2 = task::spawn(async move {
        sender.send(Either::Left(in_receiver)).await;
      });

      join!(child1, child2).await;
    })
}

pub fn offer_right
  < C, P, Q >
  ( cont:  PartialSession < C, Q >
  ) ->  PartialSession < C, InternalChoice <P, Q> >
  where
    P   : Protocol,
    Q   : Protocol,
    C : Context,
    P   : 'static,
    Q   : 'static,
    C : 'static
{
  return unsafe_create_session (
    async move |
      ctx,
      sender: Sender<
        Either<
          Receiver<P::Payload>,
          Receiver<Q::Payload> > >
    | {
      let (in_sender, in_receiver) = channel(1);

      let child1 = task::spawn(async {
        unsafe_run_session
          ( cont, ctx, in_sender
          ).await;
      });

      let child2 = task::spawn(async move {
        sender.send(Either::Right(in_receiver)).await;
      });

      join!(child1, child2).await;
    })
}

/*
  Additive Disjuction / Internal Choice

  Left Rule (Client)

      cont_builder(Left)  :: Δ, P, Δ' ⊢ S
      cont_builder(Right) :: Δ, Q, Δ' ⊢ S
  ===========================================
    case(cont_builder) :: Δ, P ⊕ Q, Δ' ⊢ S
 */

pub struct InternalChoiceResult
  < C1, C2, P >
where
  C1 : Context,
  C2 : Context,
  P : Protocol
{
  result: Either <
    PartialSession < C1, P >,
    PartialSession < C2, P >
  >
}

fn left_choice
  < C1, C2, P >
  ( res : PartialSession < C1, P > )
  ->
    InternalChoiceResult < C1, C2, P >
where
  C1 : Context,
  C2 : Context,
  P : Protocol
{
  return InternalChoiceResult {
    result: Either::Left(res)
  }
}

fn right_choice
  < C1, C2, P >
  ( res : PartialSession < C2, P > )
  ->
    InternalChoiceResult < C1, C2, P >
where
  C1 : Context,
  C2 : Context,
  P : Protocol
{
  return InternalChoiceResult {
    result: Either::Right(res)
  }
}

type ReturnChoice < N, I, P1, P2, S > =
  Either <
    Box <
      dyn FnOnce (
        PartialSession <
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          S
        >
      ) ->
        InternalChoiceResult <
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P2
            >
          > :: Target,
          S
        >
      + Send
    >,
    Box <
      dyn FnOnce (
        PartialSession <
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P2
            >
          > :: Target,
          S
        >
      ) ->
        InternalChoiceResult <
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P2
            >
          > :: Target,
          S
        >
      + Send
    >
  >;

pub fn case
  < N, I, P1, P2, S, F >
  ( _ : N,
    cont_builder : F
  ) ->
    PartialSession < I, S >
where
  I : Context,
  P1 : Protocol,
  P2 : Protocol,
  S : Protocol,
  F : FnOnce (
        ReturnChoice < N, I, P1, P2, S >
      ) ->
        InternalChoiceResult <
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          < N as
            ContextLens <
              I,
              InternalChoice < P1, P2 >,
              P2
            >
          > :: Target,
          S
        >
      + Send + 'static,
  N :
    ContextLens <
      I,
      InternalChoice < P1, P2 >,
      P1
    >,
  N :
    ContextLens <
      I,
      InternalChoice < P1, P2 >,
      P2,
      Deleted =
        < N as
          ContextLens <
            I,
            InternalChoice < P1, P2 >,
            P1
          >
        > :: Deleted
    >
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (variant_chan, ctx2) =
        < N as
          ContextLens <
            I,
            InternalChoice < P1, P2 >,
            P1
          >
        > :: extract_source ( ctx1 );

      let variant = variant_chan.recv().await.unwrap();

      match variant {
        Either::Left( p1 ) => {
          let in_choice
            : ReturnChoice < N, I, P1, P2, S >
            = Either::Left(Box::new(left_choice));

          let cont_variant = cont_builder(in_choice).result;

          let ctx3 =
            < N as
              ContextLens <
                I,
                InternalChoice < P1, P2 >,
                P1
              >
            > :: insert_target ( p1, ctx2 );

          match cont_variant {
            Either::Left(cont) => {
              unsafe_run_session
                ( cont, ctx3, sender
                ).await;
            }
            Either::Right(_) => {
              panic!("expected cont_builder to provide left result");
            }
          }
        },
        Either::Right( p2 ) => {
          let in_choice
            : ReturnChoice < N, I, P1, P2, S >
            = Either::Right(Box::new(right_choice));

          let cont_variant = cont_builder(in_choice).result;

          let ctx3 =
            < N as
              ContextLens <
                I,
                InternalChoice < P1, P2 >,
                P2
              >
            > :: insert_target ( p2, ctx2 );

          match cont_variant {
            Either::Left(_) => {
              panic!("expected cont_builder to provide right result");
            }
            Either::Right(cont) => {
              unsafe_run_session
                  ( cont, ctx3, sender).await;
              }
            }
          }
        }
      }
    )
}
