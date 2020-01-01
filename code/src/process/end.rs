use crate::base::{ Process };

use crate::process::fix::{ ProcessAlgebra };

/*
  The unit process representing termination.
 */
pub struct End {

}

impl Process for End {
  type Value = ();
}


impl < R >
  ProcessAlgebra < R >
  for End
{
  type ToProcess = End;
}