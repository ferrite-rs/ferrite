use std::marker::PhantomData;
use crate::internal::base::*;
use crate::internal::protocol::ExtractChoice;

pub trait RunCont<C, A>
where
  C: Context,
  A: Protocol,
{
  type Ret;

  fn run_cont(
    self,
    cont: PartialSession<C, A>,
  ) -> Self::Ret;
}

pub struct WrapRunCont<'a, Runner>
{
  runner: Runner,
  _phantom: PhantomData<&'a()>,
}

pub struct WrapReturn<'a, X>
{
  ret: X,
  _phantom: PhantomData<&'a()>,
}

pub fn run_cont<Runner, C, A>(
  runner: Runner,
  cont: PartialSession<C, A>,
) -> Runner::Ret
where
  C: Context,
  A: Protocol,
  Runner: RunCont<C, A>,
{
  runner.run_cont(cont)
}

pub fn unwrap_return<'a, X>
  (wrapped: WrapReturn<'a, X>)
  -> X
{
  wrapped.ret
}

impl <'a, Runner>
  WrapRunCont <'a, Runner>
{
  pub fn new(runner: Runner) -> Self {
    WrapRunCont {
      runner,
      _phantom: PhantomData
    }
  }
}

impl <'a, Ret>
  WrapReturn <'a, Ret>
{
  pub fn new(ret: Ret) -> Self {
    WrapReturn {
      ret,
      _phantom: PhantomData
    }
  }
}

impl <'a, Runner, C, A>
  RunCont<C, A>
  for WrapRunCont<'a, Runner>
where
  C: Context,
  A: Protocol,
  Runner: RunCont<C, A>
{
  type Ret = WrapReturn<'a, Runner::Ret>;

  fn run_cont(
    self,
    cont: PartialSession<C, A>,
  ) -> Self::Ret
  {
    WrapReturn::new(self.runner.run_cont(cont))
  }
}

impl <'a, Choice, Row>
  ExtractChoice<WrapRunCont<'a, Row>>
  for Choice
where
  Choice: ExtractChoice<Row>
{
  fn extract(row: WrapRunCont<'a, Row>) -> Choice {
    Choice::extract(row.runner)
  }
}
