
use crate::base as base;

use base::{ TyApp, Protocol };

pub struct End ();

impl Protocol for End { }

impl < A >
  TyApp < A >
  for End
{
  type Applied = End;
}
