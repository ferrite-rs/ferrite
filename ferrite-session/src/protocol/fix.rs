use crate::base::*;

impl < F >
  Protocol for
  Rec < F >
where
  F : Send + 'static,
{ }


impl < F >
  Protocol for
  Unfix < F >
where
  F : Send + 'static,
{ }
