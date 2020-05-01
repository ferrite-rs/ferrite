use crate::base::*;

impl < F >
  Protocol for
  Fix < F >
where
  F : Protocol + Send,
  F :
    TypeApp < Unfix <
      Fix < F >
    > >,
  F :: Applied :
    Send
{ }
