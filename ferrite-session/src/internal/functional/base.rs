use super::type_app::*;

// An implementation of Functor, Applicative, and Monad
// without resorting to HKT or GAT.

pub trait Functor: TyCon
{
  fn fmap<'a, A, B>(
    fa: App<'a, Self, A>,
    mapper: impl Fn(A) -> B,
  ) -> App<'a, Self, B>
  where
    A: 'a + Send,
    B: 'a + Send;
}

pub trait Applicative: Functor
{
  fn apply<'a, A, B, Func>(
    fab: App<'a, Self, Func>,
    fa: App<'a, Self, A>,
  ) -> App<'a, Self, B>
  where
    Func: Fn(A) -> B;
}

pub trait Monad: Applicative
{
  fn bind<'a, A, B>(
    fa: App<'a, Self, A>,
    cont: impl Fn(A) -> App<'a, Self, B>,
  ) -> App<Self, B>;
}

// NaturalTransformation f1 f2 = forall x. f1 x -> f2 x
pub trait NaturalTransformation<'a, F1, F2>
where
  F1: TyCon,
  F2: TyCon,
{
  fn lift<A: 'a>(
    self,
    fa: App<'a, F1, A>,
  ) -> App<'a, F2, A>;
}
