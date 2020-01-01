use std::marker::PhantomData;

use crate::base::{ Process };

pub trait SharedProcess {
  type SharedValue : Send;
}

pub trait SharedAlgebra < R >
{
  type ToProcess : Process;
}

pub struct Release {}

pub struct Lock < F >
{
  f : PhantomData < F >
}

pub struct LinearToShared < F >
{
  f : PhantomData < F >
}

pub struct SharedToLinear < F >
{
  f : PhantomData < F >
}
