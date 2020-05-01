
use crate::base as base;

use base::{ TypeApp, Protocol };

pub struct End ();

impl Protocol for End { }

impl < A >
  TypeApp < A >
  for End
{
  type Applied = End;
}
