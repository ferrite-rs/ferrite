
use crate::base::*;
use crate::processes::lens::*;

pub fn session
  < C, P >
  (cont : PartialSession < C, P >)
  -> Session < P >
where
  C : EmptyContext,
  P : Protocol
{
  unsafe_create_session (
    async move | (), sender | {
      let ctx = < C as EmptyContext > :: empty_values();
      unsafe_run_session ( cont, ctx, sender ).await;
    })
}

pub fn partial_session
  < C, P >
  (cont : Session < P >)
  -> PartialSession < C, P >
where
  C : EmptyContext,
  P : Protocol
{
  unsafe_create_session (
    async move |
      _,
      sender
    | {
      unsafe_run_session ( cont, (), sender ).await
    })
}

pub fn append_emtpy_slot
  < I, P >
  ( cont : PartialSession < I, P > )
  ->
    PartialSession <
      < I as
        AppendContext < ( Empty, () ) >
      > :: Appended,
      P
    >
where
  P : Protocol,
  I : Context,
  I : AppendContext < ( Empty, () ) >
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (ctx2, _) =
        < I as
          AppendContext < ( Empty, () ) >
        > :: split_context ( ctx1 );

      unsafe_run_session ( cont, ctx2, sender ).await
    })
}

pub fn session_1
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Protocol,
  F : FnOnce (Z) ->
        PartialSession <
          ( Empty, () ),
          P
        >
{
  session(cont(select_0()))
}

pub fn session_2
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Protocol,
  F : FnOnce (Z, Selector1) ->
        PartialSession <
          ( Empty, ( Empty, () )),
          P
        >
{
  session(cont(select_0(), select_1()))
}

pub fn partial_session_1
  < P1, Q, F >
  ( cont : F ) ->
    PartialSession <
      (P1, ()),
      Q
    >
where
  P1 : Slot,
  Q : Protocol,
  F : FnOnce (Z) ->
        PartialSession <
          (P1, ()),
          Q
        >
{
  cont (select_0())
}

pub fn partial_session_2
  < P1, P2, Q, F >
  ( cont : F ) ->
    PartialSession <
      (P1, (P2, ())),
      Q
    >
where
  P1 : Slot,
  P2 : Slot,
  Q : Protocol,
  F : FnOnce (Z, Selector1) ->
        PartialSession <
          (P1, (P2, ())),
          Q
        >
{
  cont (select_0(), select_1())
}
