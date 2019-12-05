use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver };

use crate::base::{ Process };

pub struct ReceiveValue < T, P >
where
  T : Send + Sized,
  P : Process
{
  value: PhantomData<T>,
  process: PhantomData<P>
}

impl
  < T, P >
  Process for
  ReceiveValue < T, P >
where
  T : Send,
  P : Process
{
  type Value = (
    Sender < T >,
    Receiver < P::Value >
  );
}
