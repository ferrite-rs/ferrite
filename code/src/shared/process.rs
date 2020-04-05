use std::marker::PhantomData;

use crate::base as base;

use base::{ Protocol };
use async_std::sync::{ Sender, Receiver };

pub trait SharedProtocol : 'static {
  type SharedValue : Send;
}

pub mod public {
  pub trait SharedProtocol : super::SharedProtocol {}
}

pub trait SharedTyApp < R >
{
  type ToProtocol : Protocol;
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
  Protocol for
  SharedToLinear < F >
where
  F : Send + 'static
{
  type Payload = ();
}

impl < F >
  Protocol for
  Lock < F >
where
  F : SharedTyApp < F >
      + Send + 'static
{
  type Payload =
    Sender <
      Receiver<
        < < F as SharedTyApp < F > >
          :: ToProtocol
          as Protocol
        > :: Payload
      >
    >;
}

impl < F >
  SharedProtocol for
  LinearToShared < F >
where
  F : SharedTyApp < F > + 'static
{
  type SharedValue =
    < < F as SharedTyApp < F > >
      :: ToProtocol
      as Protocol
    > :: Payload;
}

impl < F >
  public::SharedProtocol for
  LinearToShared < F >
where
  F : SharedTyApp < F > + 'static
{ }
