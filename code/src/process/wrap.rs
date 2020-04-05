use std::marker::PhantomData;
use crate::base as base;

use base::{ Protocol };

pub trait Wrapper {
  type Unwrap : Protocol;
}

pub struct Wrap < T > (PhantomData < T >);

impl < T >
  Protocol for
  Wrap < T >
where
  T : Send + 'static,
{
  type Payload = Box < () >;
}
