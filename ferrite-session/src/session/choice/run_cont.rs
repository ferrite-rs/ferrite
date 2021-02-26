use crate::base::*;

pub trait RunCont<C, A>
where
  C : Context,
  A : Protocol,
{
  type Ret;

  fn run_cont(
    self,
    cont : PartialSession<C, A>,
  ) -> Self::Ret;
}

pub fn run_cont<Runner, C, A>(
  runner : Runner,
  cont : PartialSession<C, A>,
) -> Runner::Ret
where
  C : Context,
  A : Protocol,
  Runner : RunCont<C, A>,
{
  runner.run_cont(cont)
}
