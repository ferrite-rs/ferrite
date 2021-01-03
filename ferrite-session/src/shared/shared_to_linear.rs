use serde;
use std::marker::PhantomData;

use crate::base as base;

use base::{ Protocol };

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SharedToLinear < F >
( pub (crate) PhantomData < F > );

impl < F > Protocol
  for SharedToLinear < F >
where
  F : Protocol
{ }
