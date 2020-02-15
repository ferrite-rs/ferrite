
use crate::base::*;
use crate::processes::lens::*;

pub fn session
  < Ins, P >
  (cont : PartialSession < Ins, P >)
  -> Session < P >
where
  Ins : EmptyContext + 'static,
  P : Protocol + 'static
{
  create_partial_session (
    async move | (), sender | {
      let ins = < Ins as EmptyContext > :: make_empty_list();
      run_partial_session ( cont, ins, sender ).await;
    })
}

pub fn partial_session
  < Ins, P >
  (cont : Session < P >)
  -> PartialSession < Ins, P >
where
  Ins : EmptyContext + 'static,
  P : Protocol + 'static
{
  create_partial_session (
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
  P : Protocol + 'static,
  I : Context + 'static,
  I : AppendContext < ( Empty, () ) >
{
  create_partial_session (
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
  P : Protocol + 'static,
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
  P : Protocol + 'static,
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
  P1 : Slot + 'static,
  Q : Protocol + 'static,
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
  P1 : Slot + 'static,
  P2 : Slot + 'static,
  Q : Protocol + 'static,
  F : FnOnce (Z, Selector1) ->
        PartialSession <
          (P1, (P2, ())),
          Q
        >
{
  cont (select_0(), select_1())
}
