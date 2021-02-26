use std::marker::PhantomData;

use super::traits::*;

pub struct InjectSessionF<N, C, B, Row, Del>(PhantomData<(N, C, B, Row, Del)>);

pub struct InjectSession<N, C, A, B, Row, Del>
{
  pub injector : Box<dyn SessionInjector<N, C, A, B, Row, Del>>,
}
