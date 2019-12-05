
use crate::base::*;
use crate::processes::lens::*;
use crate::macros::{ plist };

pub fn session
  < Ins, P >
  (cont : PartialSession < Ins, P >)
  -> Session < P >
where
  Ins : EmptyList + 'static,
  P : Process + 'static
{
  return PartialSession {
    builder: Box::new(move |
      (),
      sender
    | {
      Box::pin(async {
        let ins = < Ins as EmptyList > :: make_empty_list();
        (cont.builder)(ins, sender).await;
      })
    })
  }
}

pub fn partial_session
  < Ins, P >
  (cont : Session < P >)
  -> PartialSession < Ins, P >
where
  Ins : EmptyList + 'static,
  P : Process + 'static
{
  return PartialSession {
    builder: Box::new(move |
      _,
      sender
    | {
      (cont.builder)((), sender)
    })
  }
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
  return PartialSession {
    builder: Box::new (
      move | ins1, sender | {
        let (ins2, _) =
          < I as
            Appendable < ( Inactive, () ) >
          > :: split_channels ( ins1 );

        (cont.builder)(ins2, sender)
      })
  }
}

pub fn session_1
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Process + 'static,
  F : FnOnce (Selector1) ->
        PartialSession <
          plist![ Inactive ],
          P
        >
{
  session(cont(SELECT_1))
}

pub fn session_2
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Process + 'static,
  F : FnOnce (Selector1, Selector2) ->
        PartialSession <
          plist![ Inactive, Inactive ],
          P
        >
{
  session(cont(SELECT_1, SELECT_2))
}

pub fn session_3
  < P, F >
  ( cont : F ) ->
    Session < P >
where
  P : Process + 'static,
  F : FnOnce (Selector1, Selector2, Selector3) ->
        PartialSession <
          ( Inactive,
            ( Inactive,
              ( Inactive,
                ()
              ),
            ),
          ),
          P
        >
{
  session(cont(SELECT_1, SELECT_2, SELECT_3))
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
  F : FnOnce (Selector1) ->
        PartialSession <
          (P1, ()),
          Q
        >
{
  cont (SELECT_1)
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
  F : FnOnce (Selector1, Selector2) ->
        PartialSession <
          (P1, (P2, ())),
          Q
        >
{
  cont (SELECT_1, SELECT_2)
}

pub fn partial_session_3
  < P1, P2, P3, Q, F >
  ( cont : F ) ->
    PartialSession <
      (P1, (P2, (P3, ()))),
      Q
    >
where
  P1 : ProcessNode + 'static,
  P2 : ProcessNode + 'static,
  P3 : ProcessNode + 'static,
  Q : Process + 'static,
  F : FnOnce (Selector1, Selector2, Selector3) ->
        PartialSession <
          (P1, (P2, (P3, ()))),
          Q
        >
{
  cont (SELECT_1, SELECT_2, SELECT_3)
}
