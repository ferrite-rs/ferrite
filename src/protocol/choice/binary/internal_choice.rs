use crate::protocol::choice::nary;
use crate::protocol::choice::nary::either::*;

pub type InternalChoice < A, B > =
  nary::InternalChoice <
    Either < A, B >
  >;
