use crate::base::{ TyApp };

pub struct Val < A > {
  pub val : A
}

impl
  < A, X >
  TyApp < A > for
  Val < X >
{
  type Type = Val < X >;
}