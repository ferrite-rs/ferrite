use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };

pub struct SendChannel<P: Process, Q: Process>  {
  p: PhantomData<P>,
  q: PhantomData<Q>
}

impl <P: Process, Q: Process> Process for SendChannel<P, Q> {
  type Value = (
    Receiver< P::Value >,
    Receiver< Q::Value >
  );
}
