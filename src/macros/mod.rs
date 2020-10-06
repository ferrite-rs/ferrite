
#[macro_export]
#[allow(unused_macros)]
macro_rules! Sum {
  ( $(,)? ) => {
    $crate::choice::nary::Bottom
  };
  ( $e:ty ) => {
    $crate::choice::nary::Sum <
      $e,
      $crate::choice::nary::Bottom
    >
  };
  ( $e:ty, $($tail:tt)* ) => {
    $crate::choice::nary::Sum < $e, Sum!( $($tail)* ) >
  };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! HList {
  ( $(,)? ) => {
    ()
  };
  ( $e:ty ) => {
    ( $e, () )
  };
  ( $e:ty, $($tail:tt)* ) => {
    ( $e, HList!( $($tail)* ) )
  };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! define_choice {
  ( $label1:ident: $protocol1:ty,
    $label2:ident: $protocol2:ty
    $(,)?
  ) => {
    paste::paste! {
      pub type Protocol = HList![ $protocol1, $protocol2 ];

      pub type AsSum < A, B > = Sum![ A, B ];

      pub const [< $label1 Label >] : $crate::Z =
      $crate::Z::Value;

      pub const [< $label2 Label >] : $crate::S < $crate::Z > =
        < $crate::S < $crate::Z > >::Value;

      pub enum Branch < A, B > {
        $label1 ( A ),
        $label2 ( B ),
      }

      pub use Branch::$label1 as $label1;
      pub use Branch::$label2 as $label2;

      pub fn extract < A, B >
        ( row: Sum!( A, B ) )
        -> Branch < A, B >
      {
        match row {
          $crate::Sum::Inl ( a ) => {
            Branch::$label1 ( a )
          }
          $crate::Sum::Inr (
            $crate::Sum::Inl ( b )
          ) => {
            Branch::$label2 ( b )
          }
          $crate::Sum::Inr (
            $crate::Sum::Inr ( bot )
          ) => { match bot {} }
        }
      }
    }
  };

  ( $label1:ident: $protocol1:ty,
    $label2:ident: $protocol2:ty,
    $label3:ident: $protocol3:ty
    $(,)?
  ) => {
    paste::paste! {
      pub type Protocol =
        HList![ $protocol1, $protocol2, $protocol3 ];

      pub type AsSum < A, B, C > = Sum![ A, B, C ];

      pub const [< $label1 Label >] : $crate::Z =
      $crate::Z::Value;

      pub const [< $label2 Label >] :
        $crate::S < $crate::Z > =
        < $crate::S < $crate::Z > >::Value;

      pub const [< $label3 Label >] :
        $crate::S < $crate::S < $crate::Z > > =
        < $crate::S < $crate::S < $crate::Z > > >::Value;

      pub enum Branch < A, B,C > {
        $label1 ( A ),
        $label2 ( B ),
        $label3 ( C ),
      }

      pub use Branch::$label1 as $label1;
      pub use Branch::$label2 as $label2;
      pub use Branch::$label3 as $label3;

      pub fn extract < A, B, C >
        ( row: Sum!( A, B, C ) )
        -> Branch < A, B, C >
      {
        match row {
          $crate::Sum::Inl ( a ) => {
            Branch::$label1 ( a )
          }
          $crate::Sum::Inr (
            $crate::Sum::Inl ( b )
          ) => {
            Branch::$label2 ( b )
          }
          $crate::Sum::Inr (
            $crate::Sum::Inr (
              $crate::Sum::Inl ( c )
            )
          ) => {
            Branch::$label3 ( c )
          }
          $crate::Sum::Inr (
            $crate::Sum::Inr (
              $crate::Sum::Inr (
                bot
              )
            )
          ) => { match bot {} }
        }
      }
    }
  };
}
