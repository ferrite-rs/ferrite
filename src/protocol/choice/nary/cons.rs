use crate::base::*;
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

pub trait HasPartialSession < C, A, K >
  : Send
{
  fn with_partial_session
    ( self: Box < Self >,
      cont: Box < dyn NeedPartialSession < C, A, K > >
    ) -> K
  ;
}

pub struct WrapPartialSession < C, A > {
  session:
    Box < dyn HasPartialSession <
      C, A, Box < dyn Any > > >
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
