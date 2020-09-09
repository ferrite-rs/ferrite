use crate::protocol::choice::nary;
use crate::protocol::choice::nary::either::*;

pub type ExternalChoice < A, B > =
  nary::ExternalChoice <
    Either < A, B >
  >;
