use std::marker::PhantomData;

use crate::base as base;

use base::{ Protocol };
use async_std::sync::{ Sender, Receiver };

pub trait SharedProtocol : Send + 'static
{
  type ToLinear : Protocol;
}

pub mod public {
  pub trait SharedProtocol : super::SharedProtocol {}
}

pub trait SharedTyApp < R >
{
  type ToProtocol : Protocol;
}

pub struct Lock < F >
where
  F : SharedTyApp < F >
    + Send + 'static
{
  pub (crate) unlock:
    Sender <
      Receiver<
        F :: ToProtocol
      >
    >
}

pub struct LinearToShared < F >
( pub (crate) PhantomData < F > );

pub struct SharedToLinear < F >
( pub (crate) PhantomData < F > );

impl < F > SharedToLinear < F > {
  pub (crate) fn new () -> Self {
    SharedToLinear ( PhantomData )
  }
}

impl < F > Protocol
  for SharedToLinear < F >
where
  F : Send + 'static
{ }

impl < F >
  Protocol for
  Lock < F >
where
  F : SharedTyApp < F >
      + Send + 'static
{ }

impl < F >
  SharedProtocol for
  LinearToShared < F >
where
  F : SharedTyApp < F > + 'static + Send
{
  type ToLinear = F :: ToProtocol;
}

impl < F >
  public::SharedProtocol for
  LinearToShared < F >
where
  F : SharedTyApp < F > + 'static + Send
{ }
