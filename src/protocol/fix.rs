use crate::base::*;

impl < F >
  Protocol for
  Fix < F >
where
  F : Send + 'static,
  F :
    TypeApp < Unfix <
      Fix < F >
    > >,
  F :: Applied :
    Send
{ }


impl < F >
  Protocol for
  Unfix < F >
where
  F : Send + 'static,
{ }
