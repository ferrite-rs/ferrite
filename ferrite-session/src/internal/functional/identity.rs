use super::{
  base::*,
  type_app::*,
};

pub struct IdentityF {}

pub struct Identity<A>(pub A);

impl TyCon for IdentityF {}

impl<A> TypeApp<A> for IdentityF
where
  A : Send + 'static,
{
  type Applied = Identity<A>;
}

impl Functor for IdentityF
{
  fn fmap<A, B>(
    fa : App<IdentityF, A>,
    mapper : impl Fn(A) -> B,
  ) -> App<IdentityF, B>
  where
    A : Send + 'static,
    B : Send + 'static,
  {
    let Identity(a) = fa.get_applied();

    let b = mapper(a);

    wrap_type_app(Identity(b))
  }
}
