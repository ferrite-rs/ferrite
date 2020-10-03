use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Receiver, channel };

use crate::base::{
  TypeApp,
  TyCon,
  Applied,
  Const,
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
      row:
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
{ ctx: C::Endpoints }

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
        > >
      >
    >;

  fn lift_field < A >
    ( self,
      inject:
        impl Fn ( () ) -> Root
        + Send + 'static,
      cont:
        PartialSession < C , A >
    ) ->
      ( Receiver < A >,
        Pin < Box < dyn
          Future < Output=() >
          + Send
        > > )
  where
    A: Protocol
  {
    let (sender, receiver) = channel(1);
    let future = Box::pin(async move {
      unsafe_run_session(cont, self.ctx, sender).await;
    });

    ( receiver, future )
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
  Row : RowCon,
  Row : Send + 'static,
  Row : ElimSum,
  Row : SplitRow,
  Row : SumFunctor,
  Row :
    SumFunctorInject <
      LiftUnitToSession < C >,
      AppliedSum < Row, SessionApp < C > >,
    >,
  Row :
    SumFunctorInject <
      RunSession < C >,
      AppliedSum < Row, () >,
    >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver2) = channel(1);

      let payload = ExternalChoice::< Row >
        { sender: sender2 };

      sender1.send(payload).await;

      let (choice, sender3) = receiver2.recv().await.unwrap();

      let cont3 = Row::lift_sum_inject (
        LiftUnitToSession::<C> (PhantomData),
        | x | { x },
        choice);

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
