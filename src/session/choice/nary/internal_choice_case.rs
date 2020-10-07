use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Sender, Receiver };

use crate::base::{
  Empty,
  TyCon,
  TypeApp,
  Applied,
  Protocol,
  Context,
  wrap_applied,
  ContextLens,
  PartialSession,
  NaturalTransformation,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::protocol::choice::nary::*;

use super::internal_session::*;

pub struct InjectSessionApp < N, C, B, Row, Del >
  ( PhantomData <( N, C, B, Row, Del )> );

impl < N, C, B, Row, Del > TyCon
  for InjectSessionApp < N, C, B, Row, Del >
where
  N: 'static,
  C: 'static,
  B: 'static,
  Row: 'static,
  Del: 'static,
{}

pub trait SessionInjector
  < N, C, A, B, Row, Del >
  : Send
{
  fn inject_session
    ( self: Box < Self >,
      session:
        PartialSession <
          N :: Target,
          B
        >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del: Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del
      >,
  ;
}

pub struct InjectSession
  < N, C, A, B, Row, Del >
{
  injector: Box < dyn
    SessionInjector
    < N, C, A, B, Row, Del >
  >
}

impl < N, C, A, B, Row, Del >
  RunCont < N::Target, B >
  for InjectSession
  < N, C, A, B, Row, Del >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del: Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted = Del
    >
{
  type Ret = AppliedSum <
    Row,
    InternalSessionF < N, C, B, Row, Del >
  >;

  fn run_cont
    ( self,
      session:
        PartialSession < N :: Target, B >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
  {
    self.injector.inject_session(session)
  }
}

impl < N, C, A, B, Row, Del >
  TypeApp < A > for
  InjectSessionApp < N, C, B, Row, Del >
where
  N: Send + 'static,
  C: Send + 'static,
  A: Send + 'static,
  B: Send + 'static,
  Row: Send + 'static,
  Del: Send + 'static,
{
  type Applied =
    InjectSession <
      N, C, A, B, Row, Del
    >;
}

pub struct ReceiverToSelector {}

impl
  NaturalTransformation
  < ReceiverApp,
    Merge <
      ReceiverApp,
      ()
    >
  >
  for ReceiverToSelector
{
  fn lift < A >
    ( receiver: Applied < ReceiverApp, A > )
    ->
      Applied <
        Merge < ReceiverApp, () >,
        A
      >
  where
    A: Send + 'static,
  {
    wrap_applied ( (
      receiver,
      wrap_applied( () )
    ) )
  }
}

pub struct ContRunner1
  < N, C, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = Del,
    >,
{
  ctx : Del::Endpoints,
  sender : Sender < B >,
  phantom: PhantomData <( N, C, Row )>
}

pub struct ContRunner2
  < N, C, A, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = Del,
    >,
{
  ctx : Del::Endpoints,

  sender : Sender < B >,

  receiver : Receiver < A >,

  phantom: PhantomData <( N, C, Row )>,
}

impl < N, C, A, B, Row, Del >
  NeedInternalSession <
    N, C, A, B, Row, Del,
    Pin < Box < dyn Future < Output=() > + Send > >
  >
  for ContRunner2 < N, C, A, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N: 'static,
  N :
  ContextLens <
    C,
    InternalChoice < Row >,
    Empty,
    Deleted = Del,
  >,
{
  fn on_internal_session
    ( self: Box < Self >,
      cont: InternalSession < N, C, A, B, Row, Del >
    ) ->
      Pin < Box < dyn Future < Output=() > + Send > >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del,
      >
  {
    let ctx1 = self.ctx;
    let sender = self.sender;
    let receiver = self.receiver;

    let ctx2 =
      < N as
        ContextLens <
          C,
          InternalChoice < Row >,
          A
        >
      > :: insert_target ( receiver, ctx1 );

    Box::pin( async move {
      unsafe_run_session (
        cont.session, ctx2, sender
      ).await;
    })
  }
}

impl < B, N, C, Row, Del >
  ElimField <
    Merge <
      ReceiverApp,
      InternalSessionF < N, C, B, Row, Del >
    >,
    Pin < Box < dyn Future < Output=() > + Send > >
  > for ContRunner1 < N, C, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = Del,
    >,

{
  fn elim_field < A >
    ( self,
      fa :
        Applied <
          Merge <
            ReceiverApp,
            InternalSessionF < N, C, B, Row, Del >
          >,
          A
        >
    ) ->
      Pin < Box < dyn Future < Output=() > + Send > >
  where
    A: Send + 'static,
  {
    let (receiver1, session1) = *fa.get_applied();

    let receiver2 = *receiver1.get_applied();
    let session2 = *session1.get_applied();

    let ContRunner1 { ctx, sender, .. } = self;

    let cont = ContRunner2
      :: < N, C, A, B, Row, Del >
      {
        ctx: ctx,
        sender:sender,
        receiver: receiver2,
        phantom: PhantomData
      };

    *with_internal_session
      ( session2,
        Box::new( cont )
      )
  }
}

pub struct LiftUnitToSession < N, C, A, Row, Del >
  ( PhantomData <( N, C, A, Row, Del )> );

struct SessionInjectorImpl
  < N, C, A, B, Row, Del >
{
  injector: Box < dyn FnOnce
    ( Applied <
        InternalSessionF < N, C, B, Row, Del >,
        A
      >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
    + Send + 'static
  >
}

impl < N, C, A, B, Row, Del >
  SessionInjector
  < N, C, A, B, Row, Del >
  for SessionInjectorImpl
  < N, C, A, B, Row, Del >
{
  fn inject_session
    ( self: Box < Self >,
      session1:
        PartialSession <
          N :: Target,
          B
        >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del: Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del
      >
  {
    let session2 = wrap_internal_session ::
      < N, C, A, B, Row, Del >
      ( session1 );

    (self.injector)( wrap_applied ( session2 ) )
  }
}

impl
  < N, C, B, Row, Del >
  FieldLifter <
    AppliedSum <
      Row,
      InternalSessionF < N, C, B, Row, Del >
    >
  >
  for LiftUnitToSession < N, C, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  N : Send + 'static,
  Row : RowCon,
  InternalChoice < Row > :
    Protocol,
{
  type SourceF = ();

  type TargetF =
    InternalSessionF < N, C, B, Row, Del >;

  type InjectF =
    InjectSessionApp < N, C, B, Row, Del >;

  fn lift_field < A >
    ( self,
      inject1:
        impl Fn
          ( Applied < Self::TargetF, A > )
          ->
            AppliedSum <
              Row,
              InternalSessionF < N, C, B, Row, Del >
            >
        + Send + 'static,
      _row:
        Applied < Self::SourceF, A >
    ) ->
      Applied < Self::InjectF, A >
  where
    A: Send + 'static,
  {
    let inject2 = SessionInjectorImpl {
      injector: Box::new( inject1 )
    };

    let inject3 = InjectSession {
      injector : Box::new ( inject2 )
    };

    wrap_applied( inject3 )
  }
}

pub fn case
  < N, C, D, B, Row >
  ( _ : N,
    cont1 : impl FnOnce
      ( Row::Unwrapped )
      ->
        AppliedSum <
          Row,
          InternalSessionF < N, C, B, Row, D >
        >
      + Send + 'static,
  ) ->
    PartialSession < C, B >
where
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  Row : ElimSum,
  Row : SplitRow,
  Row : SumFunctor,
  Row : IntersectSum,
  Row : SumFunctorInject,
  Row : WrapRow < InjectSessionApp < N, C, B, Row, D > >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = D,
    >,
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (sum_chan, ctx2) =
        N::extract_source ( ctx1 );

      let InternalChoice { field : receiver_sum1 }
        = sum_chan.recv().await.unwrap();

      let sum1 = Row::lift_sum::
        < ReceiverToSelector, _, _ >
        ( receiver_sum1 );

      let (receiver_sum2, selector_sum) =
        Row::split_row(sum1);

      let cont3 :
        AppliedSum <
          Row,
          InjectSessionApp < N, C, B, Row, D >
        > =
        Row::lift_sum_inject
          ( LiftUnitToSession::
              < N, C, B, Row, D >
              (PhantomData),
            | x | { x },
            selector_sum
          );

      let cont3a = Row::unwrap_row( cont3 );

      let cont4 = cont1 ( cont3a );

      let cont5 =
        Row::intersect_sum ( receiver_sum2, cont4 );

      match cont5 {
        Some ( cont6 ) => {
          let cont7 = ContRunner1::
            < N, C, B, Row, D >
            {
              ctx: ctx2,
              sender: sender,
              phantom: PhantomData
            };

          Row::elim_sum ( cont7, cont6 ).await;
        },
        None => {
          panic!(
            "impossible happened: received mismatch choice continuation");
        }
      }
    })
}
