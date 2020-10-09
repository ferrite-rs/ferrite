use crate::base::*;
use crate::protocol::*;
use crate::functional::*;

use super::super::internal_session::*;

pub trait SessionInjector
  < N, C, A, B, Row, Del >
  : Send
{
  fn inject_session
    ( self: Box < Self >,
      session:
        PartialSession <
          N :: Target,
          B
        >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del: Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del
      >,
  ;
}
