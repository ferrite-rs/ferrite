use crate::base::{ Process };

/*
  The unit process representing termination.
 */
pub struct End {

}

impl Process for End {
  type Value = ();
}
