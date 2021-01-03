
use crate::base as base;
use serde::{Serialize, Deserialize};

use base::{ RecApp, Protocol };

#[derive(Serialize, Deserialize)]
pub struct End ();

impl Protocol for End { }

impl < A >
  RecApp < A >
  for End
{
  type Applied = End;
}
