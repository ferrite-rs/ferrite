use std::marker::PhantomData;

use crate::base as base;

use base::{ Process };
use async_std::sync::{ Sender, Receiver };

pub trait SharedProcess {
  type SharedValue : Send;
}

pub mod public {
  pub trait SharedProcess : super::SharedProcess {}
}

pub trait SharedTyCon < R >
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

impl < F >
  Process for
  SharedToLinear < F >
where
  F : Send + 'static
{
  type Value = ();
}

impl < F >
  Process for
  Lock < F >
where
  F : SharedTyCon < F >
      + Send + 'static
{
  type Value =
    Sender <
      Receiver<
        < < F as SharedTyCon < F > >
          :: ToProcess
          as Process
        > :: Value
      >
    >;
}

impl < F >
  SharedProcess for
  LinearToShared < F >
where
  F : SharedTyCon < F >
{
  type SharedValue =
    < < F as SharedTyCon < F > >
      :: ToProcess
      as Process
    > :: Value;
}

impl < F >
  base::public::Process for
  SharedToLinear < F >
where
  F : Send + 'static
{ }

impl < F >
  base::public::Process for
  Lock < F >
where
  F : SharedTyCon < F >
      + Send + 'static
{ }

impl < F >
  public::SharedProcess for
  LinearToShared < F >
where
  F : SharedTyCon < F >
{ }