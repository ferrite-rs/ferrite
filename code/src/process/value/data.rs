use crate::base::{ TyCon };

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