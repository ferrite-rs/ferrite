use std::marker::PhantomData;
use crate::base::*;

pub struct FixProtocol < F > {
  f : PhantomData < F >
}

impl < F >
  Protocol for
  FixProtocol < F >
where
  F : Protocol,
  F :: Value :
    TyApp <
      Fix < F :: Value >
    >,
  < F :: Value as
    TyApp <
      Fix < F :: Value >
    >
  > :: Type :
    Send
{
  type Value = Fix < F :: Value >;
}