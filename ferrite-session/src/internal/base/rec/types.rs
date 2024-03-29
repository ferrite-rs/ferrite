use core::marker::PhantomData;

use super::{
  HasRecApp,
  HasRecEndpoint,
};

pub struct RecX<C, F>
{
  pub unfix: Box<dyn HasRecApp<F, (RecX<C, F>, C)>>,
}

pub type Rec<F> = RecX<(), F>;

pub enum Release {}

pub struct RecRow<R, Row>
{
  phantom: PhantomData<(R, Row)>,
}

pub struct SharedRecRow<R, Row>
{
  phantom: PhantomData<(R, Row)>,
}

pub struct RecEndpoint<F, C>
{
  pub applied: Box<dyn HasRecEndpoint<F, C>>,
}
