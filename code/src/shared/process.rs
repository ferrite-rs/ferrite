use crate::base::*;
use std::marker::PhantomData;

pub trait SharedProcess {
  type SharedValue : Send;
}

pub trait ProcessAlgebra < R >
{
  type ToProcess : Process;
}

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
