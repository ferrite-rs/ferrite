use std::marker::PhantomData;

use crate::base as base;

use base::{ Protocol };
use async_std::sync::{ Sender, Receiver };

pub trait SharedProtocol : Send + 'static
{ }

pub mod public {
  pub trait SharedProtocol : super::SharedProtocol {}
}

pub trait SharedTypeApp < R >
{
  type Applied : Protocol;
}

pub struct Lock < F >
where
  F : SharedTypeApp < F >
    + Send + 'static
{
  pub (crate) unlock:
    Sender <
      Receiver<
        LinearToShared < F >
      >
    >
}

pub struct LinearToShared < F >
where
  F : SharedTypeApp < F >
{ pub (crate) linear :
    F :: Applied
}

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
  F : SharedTypeApp < F >
      + Send + 'static
{ }

impl < F >
  SharedProtocol for
  LinearToShared < F >
where
  F : SharedTypeApp < F > + 'static + Send
{ }

impl < F >
  public::SharedProtocol for
  LinearToShared < F >
where
  F : SharedTypeApp < F > + 'static + Send
{ }
