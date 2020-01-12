
use std::pin::Pin;
use std::future::Future;
use async_std::sync::Sender;

pub use crate::base::*;
pub use crate::processes::*;
pub use crate::process::nary_choice::*;

pub trait InternalSessionSum
  < Lens, I, ParentSum, Out >
  : ProcessSum
where
  ParentSum : ProcessSum,
  Out : Process,
  I : Processes,
{
  type CurrentSession : Send + 'static;

  type SessionSum : Send + 'static;
}

pub trait InternalSessionCont
  < ParentSession, Lens, I, ParentSum, Out >
  : InternalSessionSum <
      Lens, I, ParentSum, Out
    >
where
  ParentSum : ProcessSum,
  Out : Process,
  I : Processes,
  Lens :
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
        < Lens :: Deleted
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
  < Lens, I, P, Sum >
where
  P : Process,
  I : Processes,
  Sum :
    InternalSessionSum <
      Lens, I, Sum, P
    >
{
  result: Sum :: SessionSum
}

fn mk_internal_choice_result
  < Lens, I, P, Sum >
  ( session_sum : Sum :: SessionSum
  ) ->
    InternalChoiceResult <
      Lens, I, P, Sum
    >
where
  P : Process,
  I : Processes,
  Sum :
    InternalSessionSum <
      Lens, I, Sum, P
    >
{
  InternalChoiceResult {
    result : session_sum
  }
}

impl
  < Lens, I, ParentSum, Out, P >
  InternalSessionSum <
    Lens, I, ParentSum, Out
  >
  for P
where
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P
    >,
{
  type CurrentSession =
    PartialSession <
      Lens :: Target,
      Out
    >;

  type SessionSum =
    Self :: CurrentSession;
}

impl
  < Lens, I, ParentSum, Out, P, Rest >
  InternalSessionSum <
    Lens, I, ParentSum, Out
  >
  for Sum < P, Rest >
where
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum,
  Rest :
    InternalSessionSum <
      Lens, I, ParentSum, Out
    > + 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P
    >,
{
  type CurrentSession =
    PartialSession <
      Lens :: Target,
      Out
    >;

  type SessionSum =
    Sum <
      PartialSession <
        Lens :: Target,
        Out
      >,
      Rest :: SessionSum
    >;
}

impl
  < ParentSession, Lens, I, ParentSum, Out, P >
  InternalSessionCont <
    ParentSession, Lens, I, ParentSum, Out
  >
  for P
where
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum,
  ParentSession : 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      Inactive,
    >,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P,
      Deleted =
        < Lens as
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
    ( _ : SelectorZ,
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
        < < Lens as
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
      < Lens as
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
  < ParentSession, Lens, I, ParentSum, Out, P, Rest >
  InternalSessionCont <
    ParentSession, Lens, I, ParentSum, Out
  >
  for Sum < P, Rest >
where
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum,
  ParentSession : 'static,
  Rest :
    InternalSessionCont <
      ParentSession, Lens, I, ParentSum, Out
    > + 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      Inactive,
    >,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P,
      Deleted =
        < Lens as
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
        < < Lens as
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
              < Lens as
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

pub fn case
  < Lens, I, P, Sum, F >
  ( _ : Lens,
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
        Lens, I, P, Sum
      >
    + Send + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < Sum >,
      Inactive
    > + 'static,
  Sum :
    InternalSessionSum <
      Lens, I, Sum, P
    > + 'static,
  Sum :
    InternalSessionCont <
      InternalChoiceResult <
        Lens, I, P, Sum
      >,
      Lens,
      I,
      Sum,
      P
    >
{
  create_partial_session (
    async move | ins1, sender | {
      let (sum_chan, ins2) =
        < Lens as
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