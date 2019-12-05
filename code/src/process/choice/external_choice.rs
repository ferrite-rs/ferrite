use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };
use crate::process::choice::data::{ Choice, Either };

pub struct ExternalChoice<P, Q>
where
  P : Process,
  Q : Process
{
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl
  < P, Q >
  Process for ExternalChoice <P, Q>
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
