
use crate::base::*;
use crate::processes::lens::*;

pub fn session
  < Ins, P >
  (cont : PartialSession < Ins, P >)
  -> Session < P >
where
  Ins : EmptyList + 'static,
  P : Process + 'static
{
  create_partial_session (
    async move | (), sender | {
      let ins = < Ins as EmptyList > :: make_empty_list();
      run_partial_session ( cont, ins, sender ).await;
    })
}

pub fn partial_session
  < Ins, P >
  (cont : Session < P >)
  -> PartialSession < Ins, P >
where
  Ins : EmptyList + 'static,
  P : Process + 'static
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
        Appendable < ( Inactive, () ) >
      > :: AppendResult,
      P
    >
where
  P : Process + 'static,
  I : Processes + 'static,
  I : Appendable < ( Inactive, () ) >
{
  create_partial_session (
    async move | ins1, sender | {
      let (ins2, _) =
        < I as
          Appendable < ( Inactive, () ) >
        > :: split_channels ( ins1 );

      run_partial_session ( cont, ins2, sender ).await
    })
}

pub fn session_1
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Process + 'static,
  F : FnOnce (SelectorZ) ->
        PartialSession <
          ( Inactive, () ),
          P
        >
{
  session(cont(SELECT_0))
}

pub fn session_2
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Process + 'static,
  F : FnOnce (SelectorZ, Selector1) ->
        PartialSession <
          ( Inactive, ( Inactive, () )),
          P
        >
{
  session(cont(SELECT_0, SELECT_1))
}

pub fn partial_session_1
  < P1, Q, F >
  ( cont : F ) ->
    PartialSession <
      (P1, ()),
      Q
    >
where
  P1 : ProcessNode + 'static,
  Q : Process + 'static,
  F : FnOnce (SelectorZ) ->
        PartialSession <
          (P1, ()),
          Q
        >
{
  cont (SELECT_0)
}

pub fn partial_session_2
  < P1, P2, Q, F >
  ( cont : F ) ->
    PartialSession <
      (P1, (P2, ())),
      Q
    >
where
  P1 : ProcessNode + 'static,
  P2 : ProcessNode + 'static,
  Q : Process + 'static,
  F : FnOnce (SelectorZ, Selector1) ->
        PartialSession <
          (P1, (P2, ())),
          Q
        >
{
  cont (SELECT_0, SELECT_1)
}
