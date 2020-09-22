
use crate::base as base;

use base::{ RecApp, Protocol };

pub struct End ();

impl Protocol for End { }

impl < A >
  RecApp < A >
  for End
{
  type Applied = End;
}
