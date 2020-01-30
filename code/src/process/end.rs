
use crate::base as base;

use base::{ TyCon, Process };

/*
  The unit process representing termination.
 */
pub struct End {

}

impl Process for End {
  type Value = ();
}

impl base::public::Process for End {}

impl < A >
  TyCon < A >
  for End
{
  type Type = End;
}