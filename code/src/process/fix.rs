use crate::base::*;

impl < F >
  Protocol for
  Fix < F >
where
  F : Protocol + Send,
  F :
    TyApp < Unfix <
      Fix < F >
    > >,
  F :: Applied :
    Send
{ }
