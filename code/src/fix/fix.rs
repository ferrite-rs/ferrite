
pub trait AlgebraT < A >
{
  type Algebra;
}

pub struct Fix < F >
where
  F : AlgebraT < Fix < F > >
{
  unfix:
    Box <
      F :: Algebra
    >
}

pub fn fix
  < F >
  ( alg: F :: Algebra )
  -> Fix < F >
where
  F : AlgebraT < Fix < F > >
{
  Fix {
    unfix: Box::new( alg )
  }
}

pub fn unfix
  < F >
  ( f: Fix < F > )
  -> F :: Algebra
where
  F : AlgebraT < Fix < F > >
{
  *f.unfix
}
