
#[macro_export]
macro_rules! plist {
  () => {
    ()
  };
  ( $x:ty ) => {
    (
      $x,
      ()
    )
  };
  ( $x:ty, $( $y:ty ),+ ) => {
    ( $x,
      plist!( $( $y )* )
    )
  };
}

#[allow(unused_imports)]
pub (crate) use { plist };
