use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Receiver, channel };

use crate::base::{
  Protocol,
  Context,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::protocol::choice::nary::*;

pub struct SessionApp < C >
  ( PhantomData < C > );

pub struct InjectSessionApp < Root, C >
  ( PhantomData <( Root, C )> );

impl < C, A >
  RowTypeApp < A > for
  SessionApp < C >
where
  A : Protocol,
  C : Context,
{
  type Applied =
    PartialSession < C, A >;
}

impl < A, C, Root >
  RowTypeApp < A > for
  InjectSessionApp < Root, C >
where
  A : Protocol,
  C : Context,
{
  type Applied =
    InjectSession < Root, C, A >;
}

pub struct InjectSession
  < Root, C, A >
where
  A : Protocol,
  C : Context,
{
  inject_session :
    Box <
      dyn FnOnce (
        PartialSession < C, A >
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
  (inject.inject_session)(session)
}

type RootCont < C, Row > =
  InjectSessionApp <
    < Row as
      SumRow <
        SessionApp < C >
      >
    > :: Field,
    C
  >;

pub struct LiftUnitToSession < C >
  ( PhantomData< C > );

impl < Root, C >
  FieldLifterApplied < Root >
  for LiftUnitToSession < C >
{
  type Source = ();

  type Target = SessionApp < C >;

  type Injected =
    InjectSessionApp < Root, C >;
}

impl
  < Root, A, C >
  FieldLifter < Root, A >
  for LiftUnitToSession < C >
where
  A : Protocol,
  C : Context,
{
  fn lift_field (
    self,
    inject :
      impl Fn (
        PartialSession < C, A >
      ) ->
        Root
      + Send + 'static,
    _ : ()
  ) ->
    InjectSession < Root, C, A >
  {
    InjectSession {
      inject_session : Box::new ( inject )
    }
  }
}

pub struct RunSession < C >
where
  C : Context
{ ctx: C::Endpoints }

impl < Root, C >
  FieldLifterApplied < Root >
  for RunSession < C >
where
  C : Context
{
  type Source = SessionApp < C >;

  type Target = ();

  type Injected =
    Merge <
      ReceiverApp,
      Const <
        Pin < Box < dyn Future < Output=() > + Send > >
      >
    >;
}

impl
  < Root, A, C >
  FieldLifter < Root, A >
  for RunSession < C >
where
  A : Protocol,
  C : Context,
{
  fn lift_field (
    self,
    _ :
      impl Fn (
        ()
      ) ->
        Root
      + Send + 'static,
    cont: PartialSession < C, A >
  ) ->
    ( Receiver < A >,
      Pin < Box < dyn Future < Output=() > + Send > >
    )
  {
    let (sender, receiver) = channel(1);

    let future = Box::pin(unsafe_run_session(cont, self.ctx, sender));

    ( receiver,
      future
    )
  }
}

pub fn offer_choice
  < C, Row >
  ( cont1 : impl FnOnce (
      < Row as
        SumRow <
          RootCont < C, Row >
        >
      > :: Field
    ) ->
      < Row as
        SumRow <
          SessionApp < C >
        >
      > :: Field
    + Send + 'static
  ) ->
    PartialSession < C, ExternalChoice < Row > >
where
  C : Context,
  Row : SumRow <
    RootCont < C, Row >
  >,
  Row : Send + 'static,
  Row : SumRow < () >,
  Row : SumRow < ReceiverApp >,
  Row : SumRow < SessionApp < C > >,
  Row :
    LiftSum3 <
      LiftUnitToSession < C >,
      SessionApp < C >,
    >,
  Row :
    LiftSum3 <
      RunSession < C >,
      (),
    >,
  Row :
    SplitRow <
      ReceiverApp,
      Const <
        Pin < Box < dyn Future < Output=() > + Send > >
      >
    >,
  Row :
    ElimSum <
      Const <
        Pin < Box < dyn Future < Output=() > + Send > >
      >,
      ElimConst,
      Pin < Box < dyn
        Future < Output=() > + Send
      > >
    >,
{
  unsafe_create_session(
    async move | ctx, sender1 | {
      let (sender2, receiver2) = channel(1);

      let payload = ExternalChoice { sender: sender2 };

      sender1.send(payload).await;

      let (choice, sender3) = receiver2.recv().await.unwrap();

      let cont3 = Row :: lift_sum3 ( LiftUnitToSession(PhantomData), choice);

      let cont4 :
        < Row as
          SumRow <
            SessionApp < C >
          >
        > :: Field
        = cont1 ( cont3 );

      let cont5 = Row :: lift_sum3 ( RunSession { ctx: ctx }, cont4 );

      let (receiver_sum, cont6) = Row::split_row ( cont5 );

      sender3.send(receiver_sum).await;

      Row :: elim_sum ( ElimConst{}, cont6 ).await;
    })
}
