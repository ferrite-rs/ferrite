use crate::base::fix::*;

pub struct Val < A > {
  pub val : A
}

impl
  < A, X >
  TyCon < A > for
  Val < X >
{
  type Type = Val < X >;
}