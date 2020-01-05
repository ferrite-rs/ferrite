use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ Process };
use crate::process::fix::{ ProcessAlgebra };

use super::data::{ Choice, Either };

pub struct ExternalChoice
  < P, Q >
{
  p: PhantomData < P >,
  q: PhantomData < Q >
}

impl
  < P, Q >
  Process for
  ExternalChoice < P, Q >
where
  P: Process,
  Q: Process
{
  type Value = Box<
    dyn FnOnce(Choice) ->
      Either <
        Receiver < P::Value >,
        Receiver < Q::Value >
      >
    + Send >;
}

impl
  < P, Q >
  base::public::Process for
  ExternalChoice < P, Q >
where
  P: base::public::Process,
  Q: base::public::Process
{ }

impl < P, Q, R >
  ProcessAlgebra < R > for
  ExternalChoice < P, Q >
where
  P : ProcessAlgebra < R >,
  Q : ProcessAlgebra < R >,
  < P as
    ProcessAlgebra < R >
  > :: ToProcess
    : Process,
  < Q as
    ProcessAlgebra < R >
  > :: ToProcess
    : Process
{
  type ToProcess =
    ExternalChoice <
      < P as
        ProcessAlgebra < R >
      > :: ToProcess,
      < Q as
        ProcessAlgebra < R >
      > :: ToProcess
    >;
}