use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };
use crate::process::choice::data::{ Either };

/*
  data InternalChoice p q = InternalChoice
 */
pub enum InternalChoice < P, Q >
where
  P : Process,
  Q : Process
{
  PC(PhantomData<P>),
  QC(PhantomData<Q>)
}

impl
  < P, Q >
  Process for
  InternalChoice < P, Q >
where
  P: Process,
  Q: Process
{
  type Value =
    Either <
      Receiver < P::Value >,
      Receiver < Q::Value >
    >;
}
