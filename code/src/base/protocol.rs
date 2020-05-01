use crate::base::nat::*;

pub trait Protocol : Send + 'static
{ }

pub mod public {
  pub trait Protocol : super::Protocol {}
}

impl < A >
  public::Protocol
  for A
where
  A : Protocol
{}

impl Protocol for Z { }

impl < N >
  Protocol for
  S < N >
where
  N : Protocol
{ }
