use std::any::Any;
use std::marker::PhantomData;

pub trait TyCon : 'static
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

impl < T, F, A >
  HasTypeApp < F, A >
  for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  F: TypeApp < A, Applied=T >
{
  fn get_applied (self: Box < T >) -> Box < T >
  { self }
}

impl < T, F, A, K >
  TypeAppWitness < F, A, K >
  for T
where
  F: 'static,
  A: 'static,
  T: Send + 'static,
  K: 'static,
  F: TypeApp < A, Applied=T >,
{
  fn with_applied
    ( self: Box < Self >,
      cont: Box < dyn TypeAppWitnessCont < F, A, K > >
    ) -> K
  {
    cont.on_witness(self)
  }
}

pub struct Applied < F, A >
{
  pub applied:
    Box < dyn TypeAppWitness <
      F, A, Box < dyn Any > > >,
}

impl < F, A >
  Applied < F, A >
where
  F: 'static,
  A: 'static,
{
  pub fn get_applied(self)
    -> Box < F::Applied >
  where
    F: TypeApp < A >
  {
    self.applied.get_applied()
  }
}
pub fn get_applied < F, A >
  ( applied: Applied < F, A > )
  -> Box < F::Applied >
where
  F: 'static,
  A: 'static,
  F: TypeApp < A >,
{
  applied.applied.get_applied()
}

pub fn wrap_applied < F, A >
  ( applied: F::Applied )
  -> Applied < F, A >
where
  F: TypeApp < A >,
{
  Applied {
    applied: Box::new( applied )
  }
}

struct TypeAppWitnessContWrapper < F, A, K >
{
  cont: Box < dyn TypeAppWitnessCont < F, A, K > >,
}

impl < F, A, K >
  TypeAppWitnessCont < F, A, Box < dyn Any > >
  for TypeAppWitnessContWrapper < F, A, K >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  fn on_witness
    ( self: Box < Self >,
      applied: Box < F::Applied >,
    ) -> Box < dyn Any >
  where
    F: TypeApp < A >
  {
    let res = self.cont.on_witness(applied);
    Box::new(res)
  }
}

pub fn run_with_applied < F, A, K >
  ( applied: Applied < F, A >,
    cont1: Box < dyn TypeAppWitnessCont < F, A, K > >
  ) -> Box < K >
where
  F: 'static,
  A: 'static,
  K: 'static,
{
  let cont2 = TypeAppWitnessContWrapper {
    cont: cont1,
  };

  let res = applied.applied.with_applied(Box::new(cont2));
  res.downcast().unwrap()
}

pub struct Const < X > ( PhantomData<X> );

impl TyCon for () {}

impl < X > TyCon for Const < X >
where
  X: 'static
{}

impl < A > TypeApp < A >
  for ()
where
  A: 'static
{
  type Applied = ();
}

impl < X, A > TypeApp < A >
  for Const < X >
where
  A: 'static,
  X: Send + 'static,
{
  type Applied = X;
}

pub trait Functor : TyCon + Sized
{
  fn fmap < A, B >
    ( fa: Applied < Self, A >,
      mapper: impl Fn (A) -> B,
    ) ->
      Applied < Self, B >
  where
    A: Send + 'static,
    B: Send + 'static,
  ;
}

pub trait Applicative
  : Functor
{
  fn apply < A, B, Func >
    ( fab : Applied < Self, Func >,
      fa : Applied < Self, A >
    ) ->
      Applied < Self, B >
  where
    Func : Fn (A) -> B,
    A: Send + 'static,
    B: Send + 'static,
  ;
}

pub trait Monad
  : Applicative
{
  fn bind < A, B >
    ( fa : Applied < Self, A >,
      cont : impl Fn (A) -> Applied < Self, B >
    ) ->
      Applied < Self, B >
  where
    A: Send + 'static,
    B: Send + 'static,
  ;
}

pub trait NaturalTransformation < F1, F2 >
where
  F1: TyCon,
  F2: TyCon,
{
  fn lift < A >
    ( fa: Applied < F1, A > )
    -> Applied < F2, A >
  where
    A: Send + 'static,
  ;
}

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
