pub trait TyCon: Sized {}

pub trait TypeApp<'a, A>: TyCon
{
  type Applied: Sized + Send + 'a;
}

pub trait HasTypeApp<'a, F, A>: Send
{
  fn get_applied(self: Box<Self>) -> Box<F::Applied>
  where
    F: TypeApp<'a, A>;

  fn get_applied_borrow(&self) -> &F::Applied
  where
    F: TypeApp<'a, A>;
}
