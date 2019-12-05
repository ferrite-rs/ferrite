
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


#[macro_export]
macro_rules! pzipper {
  ( [$( $left:ty ),*], $p:ty, [$( $right:ty ),*]  ) => {
    ProcessZipper <
    plist!( $( $left )* ),
    $p,
    plist!( $( $right )* )
    >
  };
}


#[allow(unused_imports)]
pub (crate) use { plist, pzipper };
