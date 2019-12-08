use crate::base::{ Process };

use crate::fix::{ AlgebraT };

/*
  The unit process representing termination.
 */
pub struct End {

}

impl Process for End {
  type Value = ();
}


impl < R >
  AlgebraT < R >
  for End
{
  type Algebra = End;
}