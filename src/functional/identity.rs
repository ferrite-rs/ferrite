use super::base::*;
use super::type_app::*;

struct IdentityF {}
struct Identity < A > ( A );

impl TyCon for IdentityF {}

impl < A >
  TypeApp < A >
  for IdentityF
where
  A: Send + 'static,
{
  type Applied = Identity < A >;
}

impl
  TypeAppGeneric
  for IdentityF
{
  fn with_type_app < A, K >
    ( cont: impl TypeAppCont < IdentityF, A, K > )
    -> K
  where
    A: Send + 'static
  {
    cont.on_type_app()
  }
}

impl Functor for IdentityF
{
  fn fmap < A, B >
    ( fa: Applied < IdentityF, A >,
      mapper: impl Fn (A) -> B,
    ) ->
      Applied < IdentityF, B >
  where
    A: Send + 'static,
    B: Send + 'static,
  {
    let Identity(a) = *fa.get_applied();
    let b = mapper(a);
    wrap_applied(Identity(b))
  }
}
