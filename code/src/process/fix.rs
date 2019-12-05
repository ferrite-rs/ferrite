use std::marker::PhantomData;

use crate::fix::{ AlgebraT };
use crate::base::{ Process };

pub struct FixProcess < F > {
  f : PhantomData < F >
}

pub struct HoleProcess < F > {
  f : PhantomData < F >
}

impl < F > Process for HoleProcess < F > {
  type Value = Box < () >;
}

impl
  < F >
  Process
  for FixProcess < F >
where
  F : AlgebraT < HoleProcess < F > >,
  <
    F as AlgebraT < HoleProcess < F > >
  > :: Algebra : Process
{
  type Value = Box <
    <
      <
        F as AlgebraT < HoleProcess < F > >
      > :: Algebra
      as Process
    > :: Value
  >;
}
