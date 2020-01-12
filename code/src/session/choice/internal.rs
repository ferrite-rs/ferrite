use async_std::task;
use async_macros::join;
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{
  Either,
  InternalChoice,
};

use crate::base::{
  PartialSession,
  Process,
  Processes,
  ProcessLens,
  run_partial_session,
  create_partial_session,
};

/*
  Additive Disjuction / Internal Choice

  Right Rule (Session)

            cont :: Δ ⊢ P
  =================================
    offer_left(cont) :: Δ ⊢ P ⊕ Q

  offerLeft
    :: forall ins p q
       ( Process p
       , Process q
       , Processes ins
       )
    =>  PartialSession ins p
    ->  PartialSession ins (InternalChoice p q)
 */
pub fn offer_left
  < I, P, Q >
  ( cont:  PartialSession < I, P >
  ) ->
    PartialSession < I,
      InternalChoice < P, Q >
    >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static
{
  create_partial_session (
    async move |
      ins,
      sender: Sender<
        Either<
          Receiver<P::Value>,
          Receiver<Q::Value> > >
    | {
      let (in_sender, in_receiver) = channel(1);

      let child1 = task::spawn(async {
        run_partial_session
          ( cont, ins, in_sender
          ).await;
      });

      let child2 = task::spawn(async move {
        sender.send(Either::Left(in_receiver)).await;
      });

      join!(child1, child2).await;
    })
}

pub fn offer_right
  < Ins, P, Q >
  ( cont:  PartialSession < Ins, Q >
  ) ->  PartialSession < Ins, InternalChoice <P, Q> >
  where
    P   : Process,
    Q   : Process,
    Ins : Processes,
    P   : 'static,
    Q   : 'static,
    Ins : 'static
{
  return create_partial_session (
    async move |
      ins,
      sender: Sender<
        Either<
          Receiver<P::Value>,
          Receiver<Q::Value> > >
    | {
      let (in_sender, in_receiver) = channel(1);

      let child1 = task::spawn(async {
        run_partial_session
          ( cont, ins, in_sender
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
  < Ins1, Ins2, P >
where
  Ins1 : Processes,
  Ins2 : Processes,
  P : Process
{
  result: Either <
    PartialSession < Ins1, P >,
    PartialSession < Ins2, P >
  >
}

fn left_choice
  < Ins1, Ins2, P >
  ( res : PartialSession < Ins1, P > )
  ->
    InternalChoiceResult < Ins1, Ins2, P >
where
  Ins1 : Processes,
  Ins2 : Processes,
  P : Process
{
  return InternalChoiceResult {
    result: Either::Left(res)
  }
}

fn right_choice
  < Ins1, Ins2, P >
  ( res : PartialSession < Ins2, P > )
  ->
    InternalChoiceResult < Ins1, Ins2, P >
where
  Ins1 : Processes,
  Ins2 : Processes,
  P : Process
{
  return InternalChoiceResult {
    result: Either::Right(res)
  }
}

type ReturnChoice < Lens, I, P1, P2, S > =
  Either <
    Box <
      dyn FnOnce (
        PartialSession <
          < Lens as
            ProcessLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          S
        >
      ) ->
        InternalChoiceResult <
          < Lens as
            ProcessLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          < Lens as
            ProcessLens <
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
          < Lens as
            ProcessLens <
              I,
              InternalChoice < P1, P2 >,
              P2
            >
          > :: Target,
          S
        >
      ) ->
        InternalChoiceResult <
          < Lens as
            ProcessLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          < Lens as
            ProcessLens <
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
  < Lens, I, P1, P2, S, F >
  ( _ : Lens,
    cont_builder : F
  ) ->
    PartialSession < I, S >
where
  I : Processes + 'static,
  P1 : Process + 'static,
  P2 : Process + 'static,
  S : Process + 'static,
  F : FnOnce (
        ReturnChoice < Lens, I, P1, P2, S >
      ) ->
        InternalChoiceResult <
          < Lens as
            ProcessLens <
              I,
              InternalChoice < P1, P2 >,
              P1
            >
          > :: Target,
          < Lens as
            ProcessLens <
              I,
              InternalChoice < P1, P2 >,
              P2
            >
          > :: Target,
          S
        >
      + Send + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < P1, P2 >,
      P1
    >,
  Lens :
    ProcessLens <
      I,
      InternalChoice < P1, P2 >,
      P2,
      Deleted =
        < Lens as
          ProcessLens <
            I,
            InternalChoice < P1, P2 >,
            P1
          >
        > :: Deleted
    >
{
  create_partial_session (
    async move | ins1, sender | {
      let (variant_chan, ins2) =
        < Lens as
          ProcessLens <
            I,
            InternalChoice < P1, P2 >,
            P1
          >
        > :: split_channels ( ins1 );

      let variant = variant_chan.recv().await.unwrap();

      match variant {
        Either::Left( p1 ) => {
          let in_choice
            : ReturnChoice < Lens, I, P1, P2, S >
            = Either::Left(Box::new(left_choice));

          let cont_variant = cont_builder(in_choice).result;

          let ins3 =
            < Lens as
              ProcessLens <
                I,
                InternalChoice < P1, P2 >,
                P1
              >
            > :: merge_channels ( p1, ins2 );

          match cont_variant {
            Either::Left(cont) => {
              run_partial_session
                ( cont, ins3, sender
                ).await;
            }
            Either::Right(_) => {
              panic!("expected cont_builder to provide left result");
            }
          }
        },
        Either::Right( p2 ) => {
          let in_choice
            : ReturnChoice < Lens, I, P1, P2, S >
            = Either::Right(Box::new(right_choice));

          let cont_variant = cont_builder(in_choice).result;

          let ins3 =
            < Lens as
              ProcessLens <
                I,
                InternalChoice < P1, P2 >,
                P2
              >
            > :: merge_channels ( p2, ins2 );

          match cont_variant {
            Either::Left(_) => {
              panic!("expected cont_builder to provide right result");
            }
            Either::Right(cont) => {
              run_partial_session
                  ( cont, ins3, sender).await;
              }
            }
          }
        }
      }
    )
}