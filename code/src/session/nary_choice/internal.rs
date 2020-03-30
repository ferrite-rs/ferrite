
use std::pin::Pin;
use async_std::task;
use async_macros::join;
use std::future::Future;
use async_std::sync::{ Sender, channel };

pub use crate::base::*;
pub use crate::processes::*;
pub use crate::process::nary_choice::*;

pub trait OfferSum
  < I, N >
  : SelectSum < N >
{

}

pub trait InternalSessionSum
  < N, I, ParentSum, Out >
  : ProtocolSum2
where
  ParentSum : ProtocolSum2,
  Out : Protocol,
  I : Context,
{
  type CurrentSession : Send + 'static;

  type SessionSum : Send + 'static;
}

pub trait InternalSessionCont
  < ParentSession, N, I, ParentSum, Out >
  : InternalSessionSum <
      N, I, ParentSum, Out
    >
where
  ParentSum : ProtocolSum2,
  Out : Protocol,
  I : Context,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      Empty
    >,
{
  type CurrentCont : Send + 'static;

  type ContSum : Send + 'static;

  fn make_cont_sum
    ( selector : Self :: SelectorSum,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
          + Send
        >
    ) ->
      Self :: ContSum
  ;

  fn run_cont
    ( ins :
        < N :: Deleted
          as Context
        > :: Values,
      sender : Sender < Out :: Payload >,
      val_sum : Self :: ValueSum,
      sesssion_sum : Self :: SessionSum,
    ) ->
      Pin < Box <
        dyn Future < Output=() >
        + Send
      > >;
}

pub struct InternalChoiceResult
  < N, I, P, Sum >
where
  P : Protocol,
  I : Context,
  Sum :
    InternalSessionSum <
      N, I, Sum, P
    >
{
  result: Sum :: SessionSum
}

fn mk_internal_choice_result
  < N, I, P, Sum >
  ( session_sum : Sum :: SessionSum
  ) ->
    InternalChoiceResult <
      N, I, P, Sum
    >
where
  P : Protocol,
  I : Context,
  Sum :
    InternalSessionSum <
      N, I, Sum, P
    >
{
  InternalChoiceResult {
    result : session_sum
  }
}

impl
  < N, I, ParentSum, Out, P >
  InternalSessionSum <
    N, I, ParentSum, Out
  >
  for P
where
  P : Protocol,
  Out : Protocol,
  ParentSum : ProtocolSum2,
  I : Context,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      P
    >,
{
  type CurrentSession =
    PartialSession <
      N :: Target,
      Out
    >;

  type SessionSum =
    Self :: CurrentSession;
}

impl
  < N, I, ParentSum, Out, P, Rest >
  InternalSessionSum <
    N, I, ParentSum, Out
  >
  for Sum < P, Rest >
where
  P : Protocol,
  Out : Protocol,
  ParentSum : ProtocolSum2,
  Rest :
    InternalSessionSum <
      N, I, ParentSum, Out
    > + 'static,
  I : Context,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      P
    >,
{
  type CurrentSession =
    PartialSession <
      N :: Target,
      Out
    >;

  type SessionSum =
    Sum <
      PartialSession <
        N :: Target,
        Out
      >,
      Rest :: SessionSum
    >;
}

impl
  < ParentSession, N, I, ParentSum, Out, P >
  InternalSessionCont <
    ParentSession, N, I, ParentSum, Out
  >
  for P
where
  P : Protocol,
  Out : Protocol,
  ParentSum : ProtocolSum2,
  ParentSession : 'static,
  I : Context,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      Empty,
    >,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      P,
      Deleted =
        < N as
          ContextLens <
            I,
            InternalChoice < ParentSum >,
            Empty,
          >
        > :: Deleted
    >,
{
  type CurrentCont =
    Box <
      dyn FnOnce (
        Self :: CurrentSession
      ) ->
        ParentSession
      + Send
    >;

  type ContSum =
    Self :: CurrentCont;

  fn make_cont_sum
    ( _ : Z,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
          + Send
        >
    ) ->
      Self :: ContSum
  {
    inject
  }

  fn run_cont
    ( ins :
        < < N as
            ContextLens <
              I,
              InternalChoice < ParentSum >,
              Empty,
            >
          > :: Deleted
          as Context
        > :: Values,
      sender : Sender < Out :: Payload >,
      val : Self :: ValueSum,
      session : Self :: CurrentSession,
    ) ->
      Pin < Box <
        dyn Future < Output=() >
        + Send
      > >
  {
    let ins2 =
      < N as
        ContextLens <
          I,
          InternalChoice < ParentSum >,
          P
        >
      > :: merge_channels ( val, ins );

    Box::pin (
      run_partial_session
        ( session, ins2, sender ))
  }
}

impl
  < ParentSession, N, I, ParentSum, Out, P, Rest >
  InternalSessionCont <
    ParentSession, N, I, ParentSum, Out
  >
  for Sum < P, Rest >
where
  P : Protocol,
  Out : Protocol,
  ParentSum : ProtocolSum2,
  ParentSession : 'static,
  Rest :
    InternalSessionCont <
      ParentSession, N, I, ParentSum, Out
    > + 'static,
  I : Context,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      Empty,
    >,
  N :
    ContextLens <
      I,
      InternalChoice < ParentSum >,
      P,
      Deleted =
        < N as
          ContextLens <
            I,
            InternalChoice < ParentSum >,
            Empty,
          >
        > :: Deleted
    >,
{
  type CurrentCont =
    Box <
      dyn FnOnce (
        Self :: CurrentSession
      ) ->
        ParentSession
      + Send
    >;

  type ContSum =
    Sum <
      Self :: CurrentCont,
      Rest :: ContSum
    >;

  fn make_cont_sum
    ( selector : Self :: SelectorSum,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
          + Send
        >
    ) ->
      Self :: ContSum
  {
    match selector {
      Sum::Inl (_) => {
        let cont
          : Self :: CurrentCont
          = Box::new (
              move | session | {
                let session_sum
                  : Self :: SessionSum
                  = Sum::Inl ( session );

                let parent_session
                  : ParentSession
                  = inject ( session_sum );

                parent_session
              });

        let cont_sum
          : Self :: ContSum
          = Sum :: Inl ( cont );

        cont_sum
      },
      Sum::Inr (selector2) => {
        let inject2
          : Box <
              dyn FnOnce (
                Rest :: SessionSum
              ) ->
                ParentSession
              + Send
            >
          = Box::new (
              move | session | {
                let session_sum
                  : Self :: SessionSum
                  = Sum::Inr ( session );

                inject ( session_sum )
              });

        let cont_sum
          : Rest :: ContSum
          = Rest :: make_cont_sum (
              selector2,
              inject2
            );

        Sum :: Inr ( cont_sum )
      }
    }
  }

  fn run_cont
    ( ins :
        < < N as
            ContextLens <
              I,
              InternalChoice < ParentSum >,
              Empty,
            >
          >:: Deleted
          as Context
        > :: Values,
      sender : Sender < Out :: Payload >,
      val_sum : Self :: ValueSum,
      session_sum : Self :: SessionSum,
    ) ->
      Pin < Box <
        dyn Future < Output=() >
        + Send
      > >
  {
    match val_sum {
      Sum::Inl (val) => {
        match session_sum {
          Sum::Inl (session) => {
            let ins2 =
              < N as
                ContextLens <
                  I,
                  InternalChoice < ParentSum >,
                  P
                >
              > :: merge_channels ( val, ins );

            Box::pin (
              run_partial_session
                ( session, ins2, sender ))
          },
          Sum::Inr (_) => {
            panic!(
              "impossible happened: received mismatch value_sum");
          }
        }
      },
      Sum::Inr (val_sum2) => {
        match session_sum {
          Sum::Inl (_) => {
            panic!(
              "impossible happened: received mismatch value_sum");
          },
          Sum::Inr (session_sum2) => {
            Rest :: run_cont
              ( ins, sender,
                val_sum2, session_sum2
              )
          }
        }
      }
    }
  }
}

pub fn offer_case
  < Selector, I, Sum >
  (  _ : Selector,
    cont :
      PartialSession <
        I,
        Sum :: SelectedProtocol
      >
  ) ->
    PartialSession <
      I,
      InternalChoice < Sum >
    >
where
  I : Context,
  Sum :
    SelectSum < Selector >
    + 'static
{
  unsafe_create_session (
    async move | ins, sender1 | {
      let (sender2, receiver2) = channel (1);

      let child1 = task::spawn(async {
        run_partial_session
          ( cont, ins, sender2
          ).await;
      });

      let child2 = task::spawn(async move {
        sender1.send(
          Sum :: inject_selected ( receiver2 )
        ).await;
      });

      join!(child1, child2).await;
    }
  )
}

pub fn case
  < N, I, P, Sum, F >
  ( _ : N,
    cont_builder : F
  ) ->
    PartialSession < I, P >
where
  P : Protocol,
  I : Context,
  F :
    FnOnce (
      Sum :: ContSum
    ) ->
      InternalChoiceResult <
        N, I, P, Sum
      >
    + Send + 'static,
  N :
    ContextLens <
      I,
      InternalChoice < Sum >,
      Empty
    > + 'static,
  Sum :
    InternalSessionSum <
      N, I, Sum, P
    > + 'static,
  Sum :
    InternalSessionCont <
      InternalChoiceResult <
        N, I, P, Sum
      >,
      N,
      I,
      Sum,
      P
    >
{
  unsafe_create_session (
    async move | ins1, sender | {
      let (sum_chan, ins2) =
        < N as
          ContextLens <
            I,
            InternalChoice < Sum >,
            Empty
          >
        > :: split_channels ( ins1 );

      let val_sum = sum_chan.recv().await.unwrap();

      let selector =
        Sum ::
        value_sum_to_selector_sum
        ( &val_sum );

      let cont_sum =
        Sum :: make_cont_sum
          ( selector,
            Box::new ( mk_internal_choice_result )
          );

      let session_sum =
        cont_builder (cont_sum).result;

      Sum :: run_cont
        ( ins2,
          sender,
          val_sum,
          session_sum
        ).await
    })
}
