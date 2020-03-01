
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
      let ins = < C as EmptyContext > :: empty_values();
      run_partial_session ( cont, ins, sender ).await;
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
      run_partial_session ( cont, (), sender ).await
    })
}

pub fn append_emtpy_slot
  < I, P >
  ( cont : PartialSession < I, P > )
  ->
    PartialSession <
      < I as
        AppendContext < ( Empty, () ) >
      > :: AppendResult,
      P
    >
where
  P : Protocol,
  I : Context,
  I : AppendContext < ( Empty, () ) >
{
  unsafe_create_session (
    async move | ins1, sender | {
      let (ins2, _) =
        < I as
          AppendContext < ( Empty, () ) >
        > :: split_channels ( ins1 );

      run_partial_session ( cont, ins2, sender ).await
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
