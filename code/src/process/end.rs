
use crate::base as base;
use crate::process as process;

use base::{ Process };
use process::fix::{ ProcessAlgebra };

/*
  The unit process representing termination.
 */
pub struct End {

}

impl Process for End {
  type Value = ();
}

impl base::public::Process for End {}

impl < R >
  ProcessAlgebra < R >
  for End
{
  type ToProcess = End;
}