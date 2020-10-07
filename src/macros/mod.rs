
#[macro_export]
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
    $crate::choice::nary::Sum < $e, Sum!( $( $tail )* ) >
  };
}

#[macro_export]
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
macro_rules! match_choice {
  ( $choice:expr; $( $label:path => $e:expr $(,)? )+ ) => {
    match $crate::extract( $choice ) {
      $(
        $label ( cont ) => {
          $crate::run_cont ( cont, $e )
        }
      )*
    }
  }
}

#[macro_export]
macro_rules! define_choice_protocol {
  ( $name:ident; $( $protocols:ty ),+ $(,)? ) => {
    pub type $name =
        HList![ $( $protocols ),* ];
  }
}

#[macro_export]
macro_rules! define_choice_labels {
  ( $( $labels:ident ),+ $(,)? ) => {
    define_choice_labels![ $crate::Z; $( $labels ),* ];
  };
  ( $acc:ty; $label:ident ) => {
    paste::paste! {
      pub const [< $label Label >] : $acc =
        < $acc >::Value;
    }
  };
  ( $acc:ty; $label:ident, $( $labels:ident ),+ ) => {
    paste::paste! {
      pub const [< $label Label >] : $acc = < $acc >::Value;

      define_choice_labels![ $crate::S < $acc >; $( $labels ),* ];
    }
  };
}

#[macro_export]
macro_rules! define_choice_enum {
  ( $name:ident; $( $labels:ident ),+ $(,)? ) => {
    paste::paste! {
      pub enum [< $name Choice >]
        < $( [< $labels T >] ),* >
      {
        $( $labels ( [< $labels T >] ) ),*
      }

      pub use [< $name Choice >] :: {
        $( $labels ),*
      };
    }
  }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! match_extract {
  ( $x:ident ;
  ) => {
    match $x {}
  };
  ( $x:ident ;
    $label:ident
  ) => {
    paste::paste! {
      match $x {
        $crate::Sum::Inl ( [< $label:snake >] ) => {
          $label ( [< $label:snake >] )
        }
        $crate::Sum::Inr ( bot ) => {
          match bot { }
        }
      }
    }
  };
  ( $x:ident ;
    $label:ident, $( $labels:ident ),* $(,)?
  ) => {
    paste::paste! {
      match $x {
        $crate::Sum::Inl ( [< $label:snake >] ) => {
          $label ( [< $label:snake >] )
        }
        $crate::Sum::Inr ( [< $label:snake _rest >] ) => {
          match_extract! {
            [< $label:snake _rest >] ;
            $( $labels ),*
          }
        }
      }
    }
  };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! define_extract_choice {
  ( $name:ident ;
    $( $labels:ident ),* $(,)?
  ) => {
    paste::paste! {
      impl < $( [< $labels T >] ),* >
        $crate::ExtractRow <
          [< $name Choice >]
          < $( [< $labels T >] ),* >
        >
        for Sum![ $( [< $labels T >] ),* ]
      {
        fn extract (self) ->
          [< $name Choice >]
          < $( [< $labels T >] ),* >
        {
          match_extract! {
            self ;
            $( $labels ),*
          }
        }
      }
    }
  }
}

#[macro_export]
macro_rules! define_choice {
  ( $name:ident ;
    $( $labels:ident : $protocols:ty ),+
    $(,)?
  ) => {
    define_choice_protocol![ $name ;
      $( $protocols ),*
    ];

    define_choice_labels![
      $( $labels ),*
    ];

    define_choice_enum![ $name ;
      $( $labels ),*
    ];

    define_extract_choice![ $name ;
      $( $labels ),*
    ];
  }
}
