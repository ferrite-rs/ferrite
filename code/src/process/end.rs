
use crate::base as base;

use base::{ TyApp, Protocol };

/*
  The unit process representing termination.
 */
pub struct End {

}

impl Protocol for End {
  type Value = ();
}

impl base::public::Protocol for End {}

impl < A >
  TyApp < A >
  for End
{
  type Type = End;
}