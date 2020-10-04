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

pub struct InjectSessionApp < Root, C >
  ( PhantomData <( Root, C )> );

impl < Root, C > TyCon
  for InjectSessionApp < Root, C >
where
  C: 'static,
  Root: 'static,
{}

impl < A, C, Root >
  TypeApp < A > for
  InjectSessionApp < Root, C >
where
  C: Context,
  A: 'static,
  Root: 'static,
{
  type Applied =
    InjectSession < Root, C, A >;
}

pub struct InjectSession
  < Root, C, A >
where
  C: Context
{
  inject_session :
    Box <
      dyn FnOnce (
        Applied < SessionApp < C >, A >
      ) ->
        Root
      + Send
    >
}

pub fn run_external_cont
  < C, A, Root >
  ( inject :
      InjectSession < Root, C, A >,
    session :
      PartialSession < C, A >
  ) ->
    Root
where
  A : Protocol,
  C : Context,
{
  (inject.inject_session) (
    wrap_applied (
      wrap_session (
        session ) ) )
}

type RootCont < C, Row > =
  InjectSessionApp <
    AppliedSum <
      Row,
      SessionApp < C >
    >,
    C
  >;

pub struct LiftUnitToSession < C >
  ( PhantomData< C > );

impl
  < Root, C >
  FieldLifter < Root >
  for LiftUnitToSession < C >
where
  C: Context,
  Root: 'static,
{
  type SourceF = ();

  type TargetF = SessionApp < C >;

  type InjectF =
    InjectSessionApp < Root, C >;

  fn lift_field < A >
    ( self,
      inject:
        impl Fn
          ( Applied < Self::TargetF, A > )
          -> Root
        + Send + 'static,
      _row:
        Applied < Self::SourceF, A >
    ) ->
      Applied < Self::InjectF, A >
  where
    A: 'static,
  {
    wrap_applied (
      InjectSession {
        inject_session : Box::new ( inject )
      } )
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
  ( cont1 : impl FnOnce (
      AppliedSum <
        Row,
        RootCont < C, Row >
      >
    ) ->
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
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver2) = channel(1);

      let payload = ExternalChoice::< Row >
        { sender: sender2 };

      sender1.send(payload).await;

      let (choice, sender3) = receiver2.recv().await.unwrap();

      let cont3 = Row::lift_sum_inject
        ( LiftUnitToSession::<C> (PhantomData),
          | x | { x },
          choice
        );

      let cont4 = cont1 ( cont3 );

      let cont5 = Row::lift_sum_inject (
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
