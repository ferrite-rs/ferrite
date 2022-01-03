use super::{
  base::*,
  type_app::*,
};

pub struct IdentityF {}

pub struct Identity<A>(pub A);

impl TyCon for IdentityF {}

impl<'a, A: 'a + Send> TypeApp<'a, A> for IdentityF
{
  type Applied = Identity<A>;
}

impl Functor for IdentityF
{
  fn fmap<'a, A, B>(
    fa: App<'a, Self, A>,
    mapper: impl Fn(A) -> B,
  ) -> App<'a, Self, B>
  where
    A: 'a + Send,
    B: 'a + Send,
  {
    let Identity(a) = fa.get_applied();

    let b = mapper(a);

    wrap_type_app(Identity(b))
  }
}
