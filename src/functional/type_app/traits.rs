
pub trait TyCon : Sized + 'static
{ }

pub trait TypeApp < A > : TyCon
where
  A: 'static,
{
  type Applied: Send + 'static;
}

pub trait HasTypeApp < F, A >
  : Send
where
  F: 'static,
  A: 'static,
{
  fn get_applied
    ( self: Box < Self > )
    -> Box < F::Applied >
  where
    F: TypeApp < A >
  ;
}

pub trait TypeAppWitnessCont < F, A, K >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  fn on_witness
    ( self: Box < Self >,
      applied: Box < F::Applied >
    )
    -> K
  where
    F: TypeApp < A >
  ;
}

pub trait TypeAppWitness < F, A, K >
  : HasTypeApp < F, A >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  fn with_applied
    ( self: Box < Self >,
      cont: Box < dyn TypeAppWitnessCont < F, A, K > >
    ) -> K
  ;
}

pub trait TypeAppCont < F, A, K >
{
  fn on_type_app (self)
    -> K
  where
    A: 'static,
    F: TypeApp < A >
  ;
}

pub trait TypeAppGeneric : TyCon
{
  fn with_type_app < A, K >
    ( cont: impl TypeAppCont < Self, A, K > )
    -> K
  where
    A: Send + 'static
  ;
}
