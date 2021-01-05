use crate::base::*;
use crate::functional::*;

pub struct ReceiverF {}
pub struct SenderF {}

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

impl TyCon for ReceiverF {}
impl TyCon for SenderF {}

impl < P > TypeApp < P > for ReceiverF
where
  P: Send + 'static
{
  type Applied = ReceiverOnce < P >;
}

impl < P > TypeApp < P > for SenderF
where
  P: Send + 'static
{
  type Applied = SenderOnce < P >;
}
