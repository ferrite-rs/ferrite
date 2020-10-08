use crate::base::*;
use crate::functional::*;

use std::any::Any;
use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

pub struct ReceiverApp {}
pub struct SenderApp {}
pub struct SessionApp < C > {
  phantom: PhantomData < C >
}

pub trait NeedPartialSession < C, A, K >
{
  fn on_partial_session
    ( self: Box < Self >,
      session: PartialSession < C, A >
    ) -> K
  where
    C: Context,
    A: Protocol,
  ;
}

pub trait HasPartialSession < C, A >
  : Send
{
  fn get_partial_session
    ( self: Box < Self > )
    -> PartialSession < C, A >
  where
    C: Context,
    A: Protocol,
  ;
}

pub trait PartialSessionWitness < C, A, K >
  : HasPartialSession < C, A >
{
  fn with_partial_session
    ( self: Box < Self >,
      cont: Box < dyn NeedPartialSession < C, A, K > >
    ) -> K
  ;
}

impl < C, A >
  HasPartialSession < C, A >
  for PartialSession < C, A >
where
  C: Context,
  A: Protocol,
{
  fn get_partial_session
    ( self: Box < Self > )
    -> PartialSession < C, A >
  {
    *self
  }
}

impl < C, A, K >
  PartialSessionWitness < C, A, K >
  for PartialSession < C, A >
where
  C: Context,
  A: Protocol,
{
  fn with_partial_session
    ( self: Box < Self >,
      cont: Box < dyn NeedPartialSession < C, A, K > >
    ) -> K
  {
    cont.on_partial_session(*self)
  }
}

pub struct WrapPartialSession < C, A > {
  session:
    Box < dyn PartialSessionWitness <
      C, A, Box < dyn Any > > >
}

pub fn wrap_session < C, A >
  ( session: PartialSession < C, A > )
  -> WrapPartialSession < C, A >
where
  C: Context,
  A: Protocol,
{
  WrapPartialSession {
    session: Box::new( session )
  }
}

impl < C, A >
  WrapPartialSession < C, A >
where
  C: Context,
  A: Protocol,
{
  pub fn get_session( self )
    -> PartialSession < C, A >
  {
    self.session.get_partial_session()
  }
}

struct SessionContWrapper < C, A, K >
{
  cont: Box < dyn NeedPartialSession < C, A, K > >
}

impl < C, A, K >
  NeedPartialSession < C, A, Box < dyn Any > >
  for SessionContWrapper < C, A, K >
where
  K: 'static,
{
  fn on_partial_session
    ( self: Box < Self >,
      session: PartialSession < C, A >
    ) -> Box < dyn Any >
  where
    C: Context,
    A: Protocol,
  {
    let res = self.cont.on_partial_session( session );
    Box::new( res )
  }
}

pub fn with_session < C, A, K >
  ( session: WrapPartialSession < C, A >,
    cont1: Box < dyn NeedPartialSession < C, A, K > >
  ) ->
    Box < K >
where
  C: 'static,
  A: 'static,
  K: 'static,
{
  let cont2 = SessionContWrapper {
    cont: cont1,
  };

  let res = session.session.with_partial_session(
    Box::new( cont2 ) );

  res.downcast().unwrap()
}

pub trait RunCont < C, A >
where
  C: Context,
  A: Protocol
{
  type Ret;

  fn run_cont (
    self,
    cont: PartialSession < C, A >
  ) ->
    Self::Ret
  ;
}

pub fn run_cont < Runner, C, A >
  ( runner: Runner,
    cont: PartialSession < C, A >
  ) ->
    Runner::Ret
where
  C: Context,
  A: Protocol,
  Runner: RunCont < C, A >
{
  runner.run_cont(cont)
}

impl TyCon for ReceiverApp {}
impl TyCon for SenderApp {}

impl < C > TyCon
  for SessionApp < C >
where
  C: Send + 'static
{ }

impl < P > TypeApp < P > for ReceiverApp
where
  P: Send + 'static
{
  type Applied = Receiver < P >;
}

impl < P > TypeApp < P > for SenderApp
where
  P: Send + 'static
{
  type Applied = Sender < P >;
}

impl < C, A > TypeApp < A >
  for SessionApp < C >
where
  C: Send + 'static,
  A: Send + 'static,
{
  type Applied = WrapPartialSession < C, A >;
}
