use std::mem::transmute;
use std::marker::PhantomData;

// Type -> Type
pub trait TyCon {}

// (Type -> Type) -> Type
pub trait HigherTyCon {}

pub trait TypeApp < A > : TyCon
{ type Applied; }

pub struct Applied < F, A >
where
  F: TyCon
{ wrapped: Box < ( F, A ) > }

impl < F, A >
  Applied < F, A >
where
  F: TypeApp < A >
{ pub fn unwrap
    ( self )
    -> F::Applied
  { unsafe {
      let unwrapped : Box < F::Applied > =
        transmute( self.wrapped );
      *unwrapped
    } }
}

pub struct Const < X > ( PhantomData<X> );

impl TyCon for () {}
impl < X > TyCon for Const < X > {}

impl < A > TypeApp < A > for ()
{ type Applied = (); }

impl < X, A > TypeApp < A > for Const < X >
{ type Applied = X; }

pub fn wrap_applied < F, A >
  ( applied: F::Applied )
  -> Applied < F, A >
where
  F: TypeApp < A >
{ unsafe {
    let wrapped : Box < ( F, A ) > =
      transmute( Box::new( applied ) );
    Applied { wrapped: wrapped }
  } }

pub trait Functor : TyCon + Sized
{
  fn fmap < A, B >
    ( fa: Applied < Self, A >,
      mapper: impl Fn (A) -> B,
    ) ->
      Applied < Self, B >;
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
  ;
}

pub trait NaturalTransformationBorrow < F1, F2 >
where
  F1: TyCon,
  F2: TyCon,
{
  fn lift_borrow < A >
    ( fa: &Applied < F1, A > )
    -> Applied < F2, A >
  ;
}

struct IdentityF {}
struct Identity < A > ( A );

impl TyCon for IdentityF {}

impl < A >
  TypeApp < A >
  for IdentityF
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
  {
    let Identity(a) = fa.unwrap();
    let b = mapper(a);
    wrap_applied(Identity(b))
  }
}
