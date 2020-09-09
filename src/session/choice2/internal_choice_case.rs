use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Sender, Receiver };

pub use crate::base::{
  Nat,
  Z,
  Empty,
  TypeApp,
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

pub use crate::context::*;
pub use crate::protocol::choice2::*;

pub struct SessionApp < N, C, A, Row >
  ( PhantomData <( N, C, A, Row )> );

impl < N, I, P, Q, Row >
  TypeApp < P > for
  SessionApp < N, I, Q, Row >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >
{
  type Applied =
    PartialSession <
      N :: Target,
      Q
    >;
}

pub struct InjectSessionApp < N, C, A, Row, Root >
  ( PhantomData <( N, C, A, Row, Root )> );

impl < N, I, P, Q, Row, Root >
  TypeApp < P > for
  InjectSessionApp < N, I, Q, Row, Root >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  type Applied =
    InjectSession <
      N, I, P, Q, Row, Root
    >;
}

pub struct ReceiverToSelector {}

impl < A >
  LiftFieldBorrow
  < ReceiverApp, (), A >
  for ReceiverToSelector
where
  A : Protocol
{
  fn lift_field_borrow (
    _ : &Receiver < A >
  ) ->
    ()
  { () }
}

pub struct RunCont
  < N, C, A, Row >
where
  A : Protocol,
  C : Context,
  Row : Iso,
  Row : Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  < Row::Canon as
    SumRow < ReceiverApp >
  > :: Field :
    Send
{
  ctx :
    < N :: Deleted
      as Context
    > :: Endpoints,
  sender : Sender < A >
}

pub struct InjectSession
  < N, I, P, Q, Row, Root >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  inject_session :
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          Q
        >
      ) ->
        Root
      + Send
    >
}

pub fn run_internal_cont
  < N, I, P, Q, Row, Root >
(
  inject :
    InjectSession <
      N, I, P, Q, Row, Root
    >,
  session :
    PartialSession <
      N :: Target,
      Q
    >
) ->
  Root
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  (inject.inject_session)(session)
}

impl < A, B, N, C, Row >
  ElimField <
    Merge <
      ReceiverApp,
      SessionApp < N, C, B, Row >
    >,
    A,
    Pin < Box < dyn Future < Output=() > + Send > >
  > for RunCont < N, C, B, Row >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Row : Iso,
  Row : Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted =
        < N as
          ContextLens <
            C,
            InternalChoice < Row >,
            Empty
          >
        > :: Deleted
    >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  < Row::Canon as
    SumRow < ReceiverApp >
  > :: Field :
    Send
{
  fn elim_field (
    self,
    (receiver, cont) :
      ( Receiver < A >,
        PartialSession <
          < N as
            ContextLens <
              C,
              InternalChoice < Row >,
              A
            >
          > ::Target,
          B
        >
      )
  ) ->
    Pin < Box < dyn Future < Output=() > + Send > >
  {
    let ctx1 = self.ctx;
    let sender = self.sender;

    let ctx2 =
      < N as
        ContextLens <
          C,
          InternalChoice < Row >,
          A
        >
      > :: insert_target ( receiver, ctx1 );

    Box::pin(
      unsafe_run_session ( cont, ctx2, sender ) )
  }
}

pub struct LiftUnitToSession < N, C, A, Row >
  ( PhantomData <( N, C, A, Row )> );

impl
  < Root, N, I, P, Row >
  FieldLifterApplied < Root >
  for LiftUnitToSession < N, I, P, Row >
{
  type Source = ();

  type Target = SessionApp < N, I, P, Row >;

  type Injected =
    InjectSessionApp < N, I, P, Row, Root >;
}

impl
  < Root, N, I, P, Row, A >
  FieldLifter < Root, A >
  for LiftUnitToSession < N, I, P, Row >
where
  A : Protocol,
  P : Protocol,
  I : Context,
  Row : Send + 'static,
  Row : Iso,
  Row :
    Send + 'static,
  Row::Canon :
    SumRow < ReceiverApp >,
  < Row::Canon as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      A
    >
{
  fn lift_field (
    self,
    inject :
      impl Fn (
        PartialSession <
          N :: Target,
          P
        >
      ) ->
        Root
      + Send + 'static,
    _ : ()
  ) ->
    InjectSession < N, I, A, P, Row, Root >
  {
    InjectSession {
      inject_session : Box::new ( inject )
    }
  }
}

type RootCont < Row, N, C, A, Canon > =
  InjectSessionApp <
    N, C, A, Row,
    < Canon as
      SumRow <
        SessionApp < N, C, A, Row >
      >
    > :: Field
  >;

pub fn case
  < Row, N, C, A, Canon >
  ( _ : N,
    cont1 : impl FnOnce (
      < Row as
        SumRow <
          RootCont < Row, N, C, A, Canon >
        >
      > :: Field
    ) ->
      < Canon as
        SumRow <
          SessionApp < N, C, A, Row >
        >
      > :: Field
      + Send + 'static,
  ) ->
    PartialSession < C, A >
where
  A : Protocol,
  C : Context,
  Row : Send + 'static,
  Row : Iso < Canon = Canon >,
  Canon : 'static,
  Canon : SumRow < () >,
  Row : IsoRow <
    RootCont < Row, N, C, A, Canon >
  >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    > + 'static,
  Canon : SumRow < ReceiverApp >,
  Canon :
    SumRow <
      SessionApp < N, C, A, Row >
    >,
  Canon :
    LiftSumBorrow <
      ReceiverApp,
      (),
      ReceiverToSelector
    >,
  Canon :
    IntersectSum <
      ReceiverApp,
      SessionApp < N, C, A, Row >
    >,
  Canon :
    ElimSum <
      Merge <
        ReceiverApp,
        SessionApp < N, C, A, Row >
      >,
      RunCont < N, C, A, Row >,
      Pin < Box < dyn
        Future < Output=() > + Send
      > >
    >,
  Canon :
    LiftSum3 <
      LiftUnitToSession < N, C, A, Row >,
      SessionApp < N, C, A, Row >,
    >,
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (sum_chan, ctx2) =
        < N as
          ContextLens <
            C,
            InternalChoice < Row >,
            Empty
          >
        > :: extract_source ( ctx1 );

      let InternalChoice { field : receiver_sum }
        = sum_chan.recv().await.unwrap();

      let selector
        : < Canon as SumRow < () > > :: Field
        = Canon::lift_sum_borrow ( &receiver_sum );

      let cont2 =
        Canon :: lift_sum3 (
          LiftUnitToSession(PhantomData),
          selector
        );

      let cont3 =
        < Row as
          IsoRow <
            RootCont < Row, N, C, A, Canon >
          >
        > :: from_canon ( cont2 );

      let cont4 :
        < Canon as
          SumRow <
            SessionApp < N, C, A, Row >
          >
        > :: Field =
        cont1 ( cont3 );

      let cont5 :
        Option <
          < Canon as
            SumRow <
              Merge <
                ReceiverApp,
                SessionApp < N, C, A, Row >
              >
            >
          > :: Field
        > =
        Canon :: intersect ( receiver_sum, cont4 );

      match cont5 {
        Some ( cont6 ) => {
          let runner
            : RunCont < N, C, A, Row > =
            RunCont {
              ctx : ctx2,
              sender : sender
            };

          Canon :: elim_sum ( runner, cont6 ).await;
        },
        None => {
          panic!(
            "impossible happened: received mismatch choice continuation");
        }
      }
    })
}
