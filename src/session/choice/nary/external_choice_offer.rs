use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Receiver, channel };

use crate::base::{
  TypeApp,
  TyCon,
  Applied,
  Const,
  get_applied,
  Protocol,
  Context,
  wrap_applied,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::protocol::choice::nary::*;

pub struct InjectSessionApp < Row, C >
  ( PhantomData <( Row, C )> );

impl < Row, C > TyCon
  for InjectSessionApp < Row, C >
where
  C: 'static,
  Row: 'static,
{}

impl < A, C, Row >
  TypeApp < A > for
  InjectSessionApp < Row, C >
where
  C: Context,
  A: 'static,
  Row: 'static,
{
  type Applied =
    InjectSession < Row, C, A >;
}

pub trait SessionInjector
  < Row, C, A >
  : Send
{
  fn inject_session
    ( self: Box < Self >,
      session: PartialSession < C, A >
    ) ->
      AppliedSum <
        Row,
        SessionApp < C >
      >
  where
    C: Context,
    A: Protocol,
  ;
}

pub struct InjectSession
  < Row, C, A >
{
  injector:
    Box < dyn
      SessionInjector < Row, C, A >
    >
}

impl < Row, C, A >
  InjectSession < Row, C, A >
{
  pub fn run_cont
    ( self,
      session: PartialSession < C, A >
    ) ->
      AppliedSum <
        Row,
        SessionApp < C >
      >
  where
    C: Context,
    A: Protocol,
  {
    self.injector.inject_session( session )
  }
}

pub fn run_external_cont
  < Row, C, A >
  ( inject :
      InjectSession < Row, C, A >,
    session :
      PartialSession < C, A >
  ) ->
    AppliedSum <
      Row,
      SessionApp < C >
    >
where
  A : Protocol,
  C : Context,
{
  inject.injector.inject_session(session)
}

pub struct LiftUnitToSession < Row, C >
  ( PhantomData <( Row, C )> );

struct SessionInjectorImpl
  < Row, C, A >
{
  injector: Box < dyn FnOnce
    ( Applied < SessionApp < C >, A > )
    ->
      AppliedSum <
        Row,
        SessionApp < C >
      >
    + Send + 'static
  >
}

impl < Row, C, A >
  SessionInjector < Row, C, A >
  for SessionInjectorImpl < Row, C, A >
{
  fn inject_session
    ( self: Box < Self >,
      session: PartialSession < C, A >
    ) ->
      AppliedSum <
        Row,
        SessionApp < C >
      >
  where
    C: Context,
    A: Protocol,
  {
    (self.injector)(
      wrap_applied(
        wrap_session(
          session
        ) ) )
  }
}

impl
  < Row, C >
  FieldLifter <
    AppliedSum <
      Row,
      SessionApp < C >
    >
  >
  for LiftUnitToSession < Row, C >
where
  C: Context,
  Row: 'static,
{
  type SourceF = ();

  type TargetF = SessionApp < C >;

  type InjectF =
    InjectSessionApp < Row, C >;

  fn lift_field < A >
    ( self,
      inject1:
        impl Fn
          ( Applied < Self::TargetF, A > )
          ->
            AppliedSum <
              Row,
              SessionApp < C >
            >
        + Send + 'static,
      _row:
        Applied < Self::SourceF, A >
    ) ->
      Applied < Self::InjectF, A >
  where
    A: 'static,
  {
    let inject2 = SessionInjectorImpl {
      injector: Box::new(inject1)
    };

    let inject3 = InjectSession {
      injector : Box::new( inject2 )
    };

    wrap_applied(inject3)
  }
}

pub struct RunSession < C >
where
  C : Context
{
  ctx: C::Endpoints,
}

struct SessionRunner < C, A >
where
  C : Context
{
  ctx: C::Endpoints,
  phantom: PhantomData < A >
}

impl < C, A >
  NeedPartialSession <
    C, A,
    ( Receiver < A >,
      Pin < Box < dyn
        Future < Output=() >
        + Send + 'static
      > >
    )
  >
  for SessionRunner < C, A >
where
  C : Context
{
  fn on_partial_session
    ( self: Box < Self >,
      cont: PartialSession < C, A >
    ) ->
      ( Receiver < A >,
        Pin < Box < dyn
          Future < Output=() >
          + Send + 'static
        > >
      )
  where
    C: Context,
    A: Protocol,
  {
    let (sender, receiver) = channel(1);
    let future = Box::pin(async move {
      unsafe_run_session(
        cont,
        self.ctx,
        sender
      ).await;
    });

    ( receiver, future )
  }
}

impl
  < Root, C >
  FieldLifter < Root >
  for RunSession < C >
where
  C : Context,
{
  type SourceF = SessionApp < C >;

  type TargetF = ();

  type InjectF =
    Merge <
      ReceiverApp,
      Const <
        Pin < Box < dyn
          Future < Output=() >
          + Send + 'static
        > > > >
  ;

  fn lift_field < A >
  ( self,
    _inject:
      impl Fn
        ( Applied < Self::TargetF, A > )
        -> Root
      + Send + 'static,
    cont1:
      Applied < Self::SourceF, A >
  ) ->
    Applied < Self::InjectF, A >
  where
    A: Send + 'static,
  {
    let cont2 : WrapPartialSession < C, A >
      = *get_applied( cont1 );

    let runner: SessionRunner < C, A >
      = SessionRunner {
        ctx: self.ctx,
        phantom: PhantomData,
      };

    let (receiver, future) = *with_session (
      cont2,
      Box::new( runner )
    );

    wrap_applied( (
      wrap_applied(receiver),
      wrap_applied(future),
    ) )
  }
}

pub fn offer_choice
  < C, Row >
  ( cont1 : impl FnOnce
      ( Row::Unwrapped )
      ->
        AppliedSum <
          Row,
          SessionApp < C >
        >
    + Send + 'static
  ) ->
    PartialSession < C, ExternalChoice < Row > >
where
  C : Context + Send,
  Row : Send + 'static,
  Row : RowCon,
  Row : ElimSum,
  Row : SplitRow,
  Row : SumFunctor,
  Row : SumFunctorInject,
  Row : WrapRow < InjectSessionApp < Row, C > >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver2) = channel(1);

      let payload = ExternalChoice::< Row >
        { sender: sender2 };

      sender1.send(payload).await;

      let (choice, sender3) = receiver2.recv().await.unwrap();

      let cont3 :
        AppliedSum <
          Row,
          InjectSessionApp < Row, C >
        > =
        Row::lift_sum_inject
          ( LiftUnitToSession::< Row, C >(PhantomData),
            | x | { x },
            choice
          );

      let cont3a = Row::unwrap_row( cont3 );

      let cont4 = cont1 ( cont3a );

      let cont5 :
        AppliedSum <
          Row,
          Merge <
          ReceiverApp,
          Const <
            Pin < Box < dyn
              Future < Output=() >
              + Send + 'static
            > > > >
        > =
        Row::lift_sum_inject (
          RunSession { ctx: ctx },
          | x | { x },
          cont4
        );

      let (receiver_sum, cont6) = Row::split_row ( cont5 );

      sender3.send(
        receiver_sum
      ).await;

      Row :: elim_sum ( ElimConst{}, cont6 ).await;
    })
}
