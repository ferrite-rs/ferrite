use std::marker::PhantomData;
use crate::base::fix::*;
use crate::base::*;

pub struct FixProcess < F > {
  f : PhantomData < F >
}

impl < F >
  Process for
  FixProcess < F >
where
  F : Process,
  F :: Value :
    TyCon <
      Fix < F :: Value >
    >,
  < F :: Value as
    TyCon <
      Fix < F :: Value >
    >
  > :: Type :
    Send
{
  type Value = Fix < F :: Value >;
}