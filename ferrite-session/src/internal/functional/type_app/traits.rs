pub trait TyCon: Sized + 'static {}

pub trait TypeApp<A>: TyCon
where
  A: 'static,
{
  type Applied: Send + 'static;
}

pub trait HasTypeApp<F, A>: Send
where
  F: 'static,
  A: 'static,
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F: TypeApp<A>;

  fn get_applied_borrow(&self) -> &F::Applied
  where
    F: TypeApp<A>;
}
