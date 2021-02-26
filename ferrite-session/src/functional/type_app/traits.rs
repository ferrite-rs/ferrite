pub trait TyCon: Sized + 'static
{
}

pub trait TypeApp<A>: TyCon
where
  A : 'static,
{
  type Applied: Send + 'static;
}

pub trait HasTypeApp<F, A>: Send
where
  F : 'static,
  A : 'static,
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F : TypeApp<A>;

  fn get_applied_borrow<'a>(&'a self) -> &'a F::Applied
  where
    F : TypeApp<A>;

  fn get_applied_borrow_mut<'a>(&'a mut self) -> &'a mut F::Applied
  where
    F : TypeApp<A>;
}

pub trait TypeAppWitnessCont<F, A, K>
where
  F : 'static,
  A : 'static,
  K : 'static,
{
  fn on_witness(
    self: Box<Self>,
    applied : F::Applied,
  ) -> K
  where
    F : TypeApp<A>;
}

pub trait TypeAppCont<F, A, K>
{
  fn on_type_app(self: Box<Self>) -> K
  where
    A : 'static,
    F : TypeApp<A>;
}

pub trait TypeAppWitness<F, A, K>: Send + 'static
where
  F : 'static,
  A : 'static,
  K : 'static,
{
  fn with_applied(
    &self,
    cont : Box<dyn TypeAppCont<F, A, K>>,
  ) -> K;

  fn clone_witness(&self) -> Box<dyn TypeAppWitness<F, A, K>>;
}

pub trait TypeAppGeneric: TyCon
{
  fn get_witness<A, K>() -> Box<dyn TypeAppWitness<Self, A, K>>
  where
    A : Send + 'static,
    K : Send + 'static;
}
