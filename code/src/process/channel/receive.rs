use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base::{ Process };

pub struct ReceiveChannel<P: Process, Q: Process>  {
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl
  < P, Q >
  Process for
  ReceiveChannel < P, Q >
where
  P: Process,
  Q: Process
{
  type Value =
    Sender < (
      Receiver < P :: Value >,
      Sender < Q :: Value >
    ) >;
}
