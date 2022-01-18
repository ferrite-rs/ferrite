use super::type_app::*;

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
