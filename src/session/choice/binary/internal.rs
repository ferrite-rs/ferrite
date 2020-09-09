use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::protocol::choice::binary::{
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
  < C, A, B >
  ( cont:  PartialSession < C, A >
  ) ->
    PartialSession < C,
      InternalChoice < A, B >
    >
where
  A : Protocol,
  B : Protocol,
  C : Context
{
  unsafe_create_session (
    async move | ctx, sender | {
      let (in_sender, in_receiver) = channel(1);

      let child1 = task::spawn(async {
        unsafe_run_session
          ( cont, ctx, in_sender
          ).await;
      });

      let child2 = task::spawn(async move {
        sender.send(
          InternalChoice (
            Either::Left ( in_receiver )
          ) ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn offer_right
  < C, A, B >
  ( cont:  PartialSession < C, B > )
  -> PartialSession < C, InternalChoice < A, B > >
where
  A : Protocol,
  B : Protocol,
  C : Context,
{
  return unsafe_create_session (
    async move | ctx, sender | {
      let (in_sender, in_receiver) = channel(1);

      let child1 = task::spawn(async {
        unsafe_run_session
          ( cont, ctx, in_sender
          ).await;
      });

      let child2 = task::spawn(async move {
        sender.send (
          InternalChoice (
            Either::Right ( in_receiver )
          ) ).await;
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

fn left_choice
  < C1, C2, P >
  ( res : PartialSession < C1, P > )
  ->
    ContSum < C1, C2, P >
where
  C1 : Context,
  C2 : Context,
  P : Protocol
{
  return ContSum {
    result: Either::Left(res)
  }
}

fn right_choice
  < C1, C2, P >
  ( res : PartialSession < C2, P > )
  ->
    ContSum < C1, C2, P >
where
  C1 : Context,
  C2 : Context,
  P : Protocol
{
  return ContSum {
    result: Either::Right(res)
  }
}

pub struct ContSum
  < C1, C2, A >
where
  C1 : Context,
  C2 : Context,
  A : Protocol
{
  result: Either <
    PartialSession < C1, A >,
    PartialSession < C2, A >
  >
}

type InjectCont < C2, C3, B > =
  Either <
    Box <
      dyn FnOnce ( PartialSession < C2, B > )
      -> ContSum < C2, C3, B >
      + Send
    >,
    Box <
      dyn FnOnce ( PartialSession < C3, B > )
      -> ContSum < C2, C3, B >
      + Send
    >
  >;

pub fn case
  < N, C1, C2, C3, C4, A1, A2, B >
  ( _ : N,
    cont_builder : impl
      FnOnce (
        InjectCont < C2, C3, B >
      ) ->
        ContSum < C2, C3, B >
      + Send + 'static
  ) ->
    PartialSession < C1, B >
where
  C1 : Context,
  C4 : Context,
  C2 : Context,
  C3 : Context,
  A1 : Protocol,
  A2 : Protocol,
  B : Protocol,
  N :
    ContextLens <
      C1,
      InternalChoice < A1, A2 >,
      A1,
      Target = C2,
      Deleted = C4
    >,
  N :
    ContextLens <
      C1,
      InternalChoice < A1, A2 >,
      A2,
      Target = C3,
      Deleted = C4
    >
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (variant_chan, ctx2) =
        < N as
          ContextLens <
            C1,
            InternalChoice < A1, A2 >,
            A1
          >
        > :: extract_source ( ctx1 );

      let InternalChoice ( variant )
        = variant_chan.recv().await.unwrap();

      match variant {
        Either::Left( p1 ) => {
          let in_choice
            : InjectCont < C2, C3, B >
            = Either::Left(Box::new(left_choice));

          let cont_variant = cont_builder(in_choice).result;

          let ctx3 =
            < N as
              ContextLens <
                C1,
                InternalChoice < A1, A2 >,
                A1
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
            : InjectCont < C2, C3, B >
            = Either::Right(Box::new(right_choice));

          let cont_variant = cont_builder(in_choice).result;

          let ctx3 =
            < N as
              ContextLens <
                C1,
                InternalChoice < A1, A2 >,
                A2
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
