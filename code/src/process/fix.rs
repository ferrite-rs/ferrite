use std::marker::PhantomData;

use crate::base as base;

use base::{ Process };

pub trait ProcessAlgebra < R >
{
  type ToProcess : Process;
}

pub struct FixProcess < F > {
  f : PhantomData < F >
}

pub struct HoleProcess < F > {
  f : PhantomData < F >
}

pub struct Recurse { }

impl < F > Process for HoleProcess < F > {
  type Value = Box < () >;
}

impl
  < F >
  base::public::Process for
  HoleProcess < F >
{ }

impl
  < F >
  Process
  for FixProcess < F >
where
  F : ProcessAlgebra < HoleProcess < F > >
{
  type Value = Box <
    <
      <
        F as ProcessAlgebra < HoleProcess < F > >
      > :: ToProcess
      as Process
    > :: Value
  >;
}

impl
  < F >
  base::public::Process
  for FixProcess < F >
where
  F : ProcessAlgebra < HoleProcess < F > >
{ }

impl < R >
  ProcessAlgebra < R >
  for Recurse
where
  R : Process
{
  type ToProcess = R;
}