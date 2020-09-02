use std::marker::PhantomData;
use async_std::task;
use async_macros::join;
use async_std::sync::{ Receiver, channel };

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

pub struct SessionCon < C >
  ( PhantomData < C > );

pub struct ExternalCont < C, Root >
  ( PhantomData <( C, Root )> );

impl < C, A >
  TypeApp < A > for
  SessionCon < C >
where
  A : Protocol,
  C : Context,
{
  type Applied =
    PartialSession < C, A >;
}

impl < A, C, Root >
  TypeApp < A > for
  ExternalCont < C, Root >
where
  A : Protocol,
  C : Context,
{
  type Applied =
    InjectSession < C, A, Root >;
}

pub struct InjectSession
  < C, A, Root >
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
    >
}

type RootCont < C, Canon > =
  ExternalCont < C,
    < Canon as
      SumRow <
        SessionCon < C >
      >
    > :: Field
  >;

impl < Root, C >
  FieldLifterApplied < Root >
  for SessionCon < C >
{
  type Source = ();

  type Injected =
    ExternalCont < C, Root >;
}

impl
  < Root, A, C >
  FieldLifter < Root, A >
  for SessionCon < C >
where
  A : Protocol,
  C : Context,
{
  fn lift_field (
    inject :
      impl Fn (
        PartialSession < C, A >
      ) ->
        Root
      + Send + 'static,
    _ : ()
  ) ->
    InjectSession < C, A, Root >
  {
    InjectSession {
      inject_session : Box::new ( inject )
    }
  }
}

pub fn offer_case
  < C, Row, Canon >
  ( cont1 : impl FnOnce (
      < Row as
        SumRow <
          RootCont < C, Canon >
        >
      > :: Field
    ) ->
      < Canon as
        SumRow <
          SessionCon < C >
        >
      > :: Field
    + Send + 'static
  ) ->
    PartialSession < C, ExternalChoice < Row > >
where
  C : Context,
  Row : IsoRow <
    RootCont < C, Canon >
  >,
  Row : Send + 'static,
  Row : Iso < Canon = Canon >,
  Canon : 'static,
  Canon : SumRow < () >,
  Canon : SumRow < ReceiverCon >,
  Canon : SumRow < SessionCon < C > >,
  Canon : SumRow <
    ExternalCont < C,
      < Canon as
        SumRow <
          SessionCon < C >
        >
      > :: Field
    >
  >,
{
  todo!()
}
