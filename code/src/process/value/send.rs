use std::marker::PhantomData;
use async_std::sync::{ Receiver };

use crate::base::{ Process };

pub struct SendValue < T, P >
where
  T : Send + Sized,
  P : Process
{
  value: PhantomData<T>,
  process: PhantomData<P>
}

impl < T, P > Process for SendValue < T, P >
where
  T : Send,
  P : Process
{
  type Value = (
    T,
    Receiver < P::Value >
  );
}
