
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
  : ProcessSum2
where
  ParentSum : ProcessSum2,
  Out : Process,
  I : Processes,
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
  ParentSum : ProcessSum2,
  Out : Process,
  I : Processes,
  N :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      Inactive
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
          as Processes
        > :: Values,
      sender : Sender < Out :: Value >,
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
  P : Process,
  I : Processes,
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
  P : Process,
  I : Processes,
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
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum2,
  I : Processes + 'static,
  N :
    ProcessLens <
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
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum2,
  Rest :
    InternalSessionSum <
      N, I, ParentSum, Out
    > + 'static,
  I : Processes + 'static,
  N :
    ProcessLens <
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
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum2,
  ParentSession : 'static,
  I : Processes + 'static,
  N :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      Inactive,
    >,
  N :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P,
      Deleted =
        < N as
          ProcessLens <
            I,
            InternalChoice < ParentSum >,
            Inactive,
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
            ProcessLens <
              I,
              InternalChoice < ParentSum >,
              Inactive,
            >
          > :: Deleted
          as Processes
        > :: Values,
      sender : Sender < Out :: Value >,
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
        ProcessLens <
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
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum2,
  ParentSession : 'static,
  Rest :
    InternalSessionCont <
      ParentSession, N, I, ParentSum, Out
    > + 'static,
  I : Processes + 'static,
  N :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      Inactive,
    >,
  N :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P,
      Deleted =
        < N as
          ProcessLens <
            I,
            InternalChoice < ParentSum >,
            Inactive,
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
            ProcessLens <
              I,
              InternalChoice < ParentSum >,
              Inactive,
            >
          >:: Deleted
          as Processes
        > :: Values,
      sender : Sender < Out :: Value >,
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
                ProcessLens <
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
        Sum :: SelectedProcess
      >
  ) ->
    PartialSession <
      I,
      InternalChoice < Sum >
    >
where
  I : Processes + 'static,
  Sum :
    SelectSum < Selector >
    + 'static
{
  create_partial_session (
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
  P : Process + 'static,
  I : Processes + 'static,
  F :
    FnOnce (
      Sum :: ContSum
    ) ->
      InternalChoiceResult <
        N, I, P, Sum
      >
    + Send + 'static,
  N :
    ProcessLens <
      I,
      InternalChoice < Sum >,
      Inactive
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
  create_partial_session (
    async move | ins1, sender | {
      let (sum_chan, ins2) =
        < N as
          ProcessLens <
            I,
            InternalChoice < Sum >,
            Inactive
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
