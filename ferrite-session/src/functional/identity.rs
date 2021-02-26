use super::{base::*, type_app::*};

struct IdentityF {}

struct Identity<A>(A);

impl TyCon for IdentityF {}

impl<A> TypeApp<A> for IdentityF
where
  A : Send + 'static,
{
  type Applied = Identity<A>;
}

impl TypeAppGeneric for IdentityF
{
  fn get_witness<A, K>() -> Box<dyn TypeAppWitness<Self, A, K>>
  where
    A : Send + 'static,
    K : Send + 'static,
  {

    Box::new(())
  }
}

impl Functor for IdentityF
{
  fn fmap<A, B>(
    fa : Applied<IdentityF, A>,
    mapper : impl Fn(A) -> B,
  ) -> Applied<IdentityF, B>
  where
    A : Send + 'static,
    B : Send + 'static,
  {

    let Identity(a) = fa.get_applied();

    let b = mapper(a);

    cloak_applied(Identity(b))
  }
}
