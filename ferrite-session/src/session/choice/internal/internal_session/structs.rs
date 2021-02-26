use std::{
  any::Any,
  marker::PhantomData,
};

use super::traits::*;
use crate::{
  base::*,
  functional::*,
  protocol::*,
};

pub struct InternalSessionF<N, C, B, Row, Del>
{
  phantom : PhantomData<(N, C, B, Row, Del)>,
}

pub struct InternalSession<N, C, A, B, Row, Del>
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
{
  pub session : PartialSession<N::Target, B>,
}

pub struct CloakInternalSession<N, C, A, B, Row, Del>
{
  pub session :
    Box<dyn InternalSessionWitness<N, C, A, B, Row, Del, Box<dyn Any>>>,
}
