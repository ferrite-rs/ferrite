pub use paste::paste;

#[macro_export]
macro_rules! Sum {
  ( $(,)? ) => {
    $crate::choice::nary::Bottom
  };
  ( $e:ty ) => {
    $crate::prelude::Sum <
      $e,
      $crate::prelude::Bottom
    >
  };
  ( $e:ty, $($tail:tt)* ) => {
    $crate::prelude::Sum < $e, $crate::prelude::Sum!( $( $tail )* ) >
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
    ( $e, $crate::prelude::HList!( $($tail)* ) )
  };
}

#[macro_export]
macro_rules! match_choice_value {
  ( $choice:expr; $( $label:path => $e:expr $(,)? )+ ) => {
    match $crate::prelude::extract( $choice ) {
      $(
        $label ( cont ) => {
          $crate::prelude::run_cont ( cont,
            $crate::prelude::step ( async move {
              $e
            })
          )
        }
      )*
    }
  }
}

#[macro_export]
macro_rules! match_choice {
  ( $( $label:path => $e:expr $(,)? )+ ) => {
    move | ferrite_choice_internal__ | {
      $crate::match_choice_value! { ferrite_choice_internal__;
        $( $label => $e ),*
      }
    }
  };
}

#[macro_export]
macro_rules! offer_choice {
  ( $( $label:path => $e:expr $(,)? )+ ) => {
    $crate::prelude::offer_choice (
      $crate::match_choice! {
        $( $label => $e ),*
      }
    )
  }
}

#[macro_export]
macro_rules! case {
  ( $chan:expr ; $( $label:path => $e:expr $(,)? )+ ) => {
    $crate::prelude::case ( $chan,
      $crate::match_choice! {
        $( $label => $e ),*
      }
    )
  }
}

#[macro_export]
macro_rules! define_choice_protocol {
  ( $name:ident ;
    $( $protocols:ty ),+ $(,)?
  ) => {
    pub enum $name {}

    impl $crate::prelude::ToRow for $name {
      type Row = $crate::prelude::HList![ $( $protocols ),* ];
    }
  };

  ( $name:ident
    < $( $types:ident ),+ $(,)? > ;
    $( $protocols:ty ),+ $(,)?
  ) => {
    pub struct $name <$( $types ),*>
    {
      phantom: std::marker::PhantomData<($( $types ),*)>
    }

    impl < $( $types ),* >
      $crate::prelude::ToRow for $name < $( $types ),* >
    {
      type Row = $crate::prelude::HList![ $( $protocols ),* ];
    }
  };
}

#[macro_export]
macro_rules! define_choice_labels {
  ( $( $labels:ident ),+ $(,)? ) => {
    $crate::define_choice_labels![ $crate::prelude::Z; $( $labels ),* ];
  };
  ( $acc:ty; $label:ident ) => {
    $crate::macros::paste! {
      #[allow(non_upper_case_globals)]
      pub const [< $label Label >]
        : $crate::prelude::ChoiceSelector < $acc > =
        < $crate::prelude::ChoiceSelector < $acc > >::new();
    }
  };
  ( $acc:ty; $label:ident, $( $labels:ident ),+ ) => {
    $crate::macros::paste! {
      #[allow(non_upper_case_globals)]
      pub const [< $label Label >]
        : $crate::prelude::ChoiceSelector < $acc > =
        < $crate::prelude::ChoiceSelector < $acc > >::new();

      $crate::define_choice_labels![
        $crate::prelude::S < $acc >;
        $( $labels ),*
      ];
    }
  };
}

#[macro_export]
macro_rules! define_choice_enum {
  ( $name:ident; $( $labels:ident ),+ $(,)? ) => {
    $crate::macros::paste! {
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
    $crate::macros::paste! {
      match $x {
        $crate::prelude::Sum::Inl ( [< $label:snake >] ) => {
          $label ( [< $label:snake >] )
        }
        $crate::prelude::Sum::Inr ( bot ) => {
          match bot { }
        }
      }
    }
  };
  ( $x:ident ;
    $label:ident, $( $labels:ident ),* $(,)?
  ) => {
    $crate::macros::paste! {
      match $x {
        $crate::prelude::Sum::Inl ( [< $label:snake >] ) => {
          $label ( [< $label:snake >] )
        }
        $crate::prelude::Sum::Inr ( [< $label:snake _rest >] ) => {
          $crate::match_extract! {
            [< $label:snake _rest >] ;
            $( $labels ),*
          }
        }
      }
    }
  };
}

#[macro_export]
macro_rules! define_extract_choice {
  ( $name:ident ;
    $( $labels:ident ),* $(,)?
  ) => {
    $crate::macros::paste! {
      impl < $( [< $labels T >] ),* >
        std::convert::From <
          $crate::prelude::Sum![ $( [< $labels T >] ),* ]
        >
        for [< $name Choice >]
          < $( [< $labels T >] ),* >
      {
        fn from
          (row: $crate::prelude::Sum![ $( [< $labels T >] ),* ] )
          -> Self
        {
          $crate::match_extract! {
            row ;
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
    $crate::define_choice_protocol![ $name ;
      $( $protocols ),*
    ];

    $crate::define_choice_labels![
      $( $labels ),*
    ];

    $crate::define_choice_enum![ $name ;
      $( $labels ),*
    ];

    $crate::define_extract_choice![ $name ;
      $( $labels ),*
    ];
  };

  ( $name:ident
    < $( $types:ident ),+ $(,)? > ;
    $( $labels:ident : $protocols:ty ),+
    $(,)?
  ) => {
    $crate::define_choice_protocol![
      $name < $( $types ),* > ;
      $( $protocols ),*
    ];

    $crate::define_choice_labels![
      $( $labels ),*
    ];

    $crate::define_choice_enum![ $name ;
      $( $labels ),*
    ];

    $crate::define_extract_choice![ $name ;
      $( $labels ),*
    ];
  };
}

#[macro_export]
macro_rules! send_value {
  ($val:expr, $cont:expr) => {
    $crate::prelude::step(
      async move { $crate::prelude::send_value($val, $cont) },
    )
  };
}

#[macro_export]
macro_rules! send_value_to {
  ($chan:expr, $val:expr, $cont:expr) => {
    $crate::prelude::step(async move {
      $crate::prelude::send_value_to($chan, $val, $cont)
    })
  };
}

#[macro_export]
macro_rules! receive_value {
  ( $var:ident => $body:expr ) => {
    $crate::prelude::receive_value (
      move | $var | {
        $crate::prelude::step ( async move {
          $body
        })
      }
    )
  };
  ( ($var:ident $( : $type:ty )?) => $body:expr ) => {
    $crate::prelude::receive_value (
      move | $var $( : $type )* | {
        $crate::prelude::step ( async move {
          $body
        })
      }
    )
  }
}

#[macro_export]
macro_rules! receive_value_from {
  ( $chan:expr,
    $var:ident => $body:expr
  ) => {
    $crate::prelude::receive_value_from (
      $chan,
      move | $var | {
        $crate::prelude::step ( async move {
          $body
        })
      }
    )
  };
  ( $chan:expr,
    ($var:ident $( : $type:ty )?) => $body:expr
  ) => {
    $crate::prelude::receive_value_from (
      $chan,
      move | $var $( : $type )* | {
        $crate::prelude::step ( async move {
          $body
        })
      }
    )
  }
}

#[macro_export]
macro_rules! choose {
  ($chan:expr, $label:ident, $cont:expr) => {
    $crate::macros::paste! {
      $crate::prelude::choose (
        $chan,
        [< $label Label >],
        $cont
      )
    }
  };
}

#[macro_export]
macro_rules! offer_case {
  ($label:ident, $cont:expr) => {
    $crate::macros::paste! {
      $crate::prelude::offer_case (
        [< $label Label >],
        $cont
      )
    }
  };
}

#[macro_export]
macro_rules! acquire_shared_session {
  ($chan:expr, $var:ident => $body:expr) => {
    $crate::prelude::acquire_shared_session($chan.clone(), move |$var| {
      $crate::prelude::step(async move { $body })
    })
  };
}

#[macro_export]
macro_rules! receive_channel {
  ($var:ident => $body:expr) => {
    $crate::prelude::receive_channel(move |$var|
      $crate::prelude::step(async move { $body }))
  };
}

#[macro_export]
macro_rules! receive_channels {
  ( ( $var:ident $(,)? ) => $body:expr ) => {
    $crate::receive_channel!( $var => $body )
  };
  ( ( $var:ident, $( $vars:ident ),* $(,)? )
    => $body:expr
  ) => {
    $crate::receive_channel! ( $var => {
      $crate::receive_channels! (
        ( $( $vars ),* ) =>
          $body
      )
    })
  };
}

#[macro_export]
macro_rules! receive_channel_from {
  ($chan:expr, $var:ident => $body:expr) => {
    $crate::prelude::receive_channel_from($chan, move |$var| $body)
  };
}

#[macro_export]
macro_rules! include_session {
  ($session:expr, $var:ident => $body:expr) => {
    $crate::prelude::include_session($session, move |$var| {
      $crate::prelude::step(async move { $body })
    })
  };
}

#[macro_export]
macro_rules! terminate {
  () => {
    $crate::prelude::terminate()
  };
  ($cont:expr) => {
    $crate::prelude::terminate_async(move || async move { $cont })
  };
}

#[macro_export]
macro_rules! wait {
  ($chan:expr, $cont:expr) => {
    $crate::prelude::wait($chan,
      $crate::prelude::step(async move { $cont }))
  };
}

#[macro_export]
macro_rules! wait_all {
  ( [ $chan:expr $(,)? ],
    $cont:expr
  ) => {
    $crate::prelude::wait! ( $chan, $cont )
  };
  ( [ $chan:expr, $( $chans:expr ),* $(,)? ],
    $cont:expr
  ) => {
    $crate::prelude::wait! ( $chan,
      $crate::prelude::wait_all! (
        [ $( $chans ),* ],
        $cont
      )
    )
  };
}

#[macro_export]
macro_rules! cut {
  ( [ $( $labels:ty ),+ $(,)? ] ;
    $cont1:expr ;
    $var:ident => $cont2:expr
  ) => {
    < $crate::prelude::HList![ $( $labels ),* ]
      as $crate::prelude::Cut < _ >
    > :: cut (
      $cont1,
      move | $var | {
        $crate::prelude::step ( async move {
          $cont2
        })
      }
    )
  }
}
