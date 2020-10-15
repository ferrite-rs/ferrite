
#[macro_export]
macro_rules! Sum {
  ( $(,)? ) => {
    $crate::choice::nary::Bottom
  };
  ( $e:ty ) => {
    $crate::Sum <
      $e,
      $crate::Bottom
    >
  };
  ( $e:ty, $($tail:tt)* ) => {
    $crate::Sum < $e, Sum!( $( $tail )* ) >
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
macro_rules! match_choice_value {
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
macro_rules! match_choice {
  ( $( $label:path => $e:expr $(,)? )+ ) => {
    move | ferrite_choice_internal__ | async move {
      match_choice_value! { ferrite_choice_internal__;
        $( $label => $e ),*
      }
    }
  };
}

#[macro_export]
macro_rules! offer_choice {
  ( $( $label:path => $e:expr $(,)? )+ ) => {
    $crate::offer_choice (
      match_choice! {
        $( $label => $e ),*
      }
    )
  }
}

#[macro_export]
macro_rules! case {
  ( $chan:expr ; $( $label:path => $e:expr $(,)? )+ ) => {
    $crate::case ( $chan,
      match_choice! {
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
    pub type $name =
        HList![ $( $protocols ),* ];
  };

  ( $name:ident < $( $types:ident ),+ $(,)? > ;
    $( $protocols:ty ),+ $(,)?
  ) => {
    pub type $name < $( $types ),* > =
        HList![ $( $protocols ),* ];
  };
}

#[macro_export]
macro_rules! define_choice_labels {
  ( $( $labels:ident ),+ $(,)? ) => {
    define_choice_labels![ $crate::Z; $( $labels ),* ];
  };
  ( $acc:ty; $label:ident ) => {
    paste::paste! {
      #[allow(non_upper_case_globals)]
      pub const [< $label Label >]
        : $crate::ChoiceSelector < $acc > =
        < $crate::ChoiceSelector < $acc > >::new();
    }
  };
  ( $acc:ty; $label:ident, $( $labels:ident ),+ ) => {
    paste::paste! {
      #[allow(non_upper_case_globals)]
      pub const [< $label Label >]
        : $crate::ChoiceSelector < $acc > =
        < $crate::ChoiceSelector < $acc > >::new();

      define_choice_labels![
        $crate::S < $acc >;
        $( $labels ),*
      ];
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
  };

  ( $name:ident < $( $types:ident ),+ $(,)? > ;
    $( $labels:ident : $protocols:ty ),+
    $(,)?
  ) => {
    define_choice_protocol![
      $name < $( $types ),* > ;
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
  };
}


#[macro_export]
macro_rules! send_value {
  ( $val:expr, $cont:expr ) => {
    $crate::step ( move || async move {
      $crate::send_value (
        $val,
        $cont
      )
    })
  }
}


#[macro_export]
macro_rules! send_value_to {
  ( $chan:expr, $val:expr, $cont:expr ) => {
    $crate::step ( move || async move {
      $crate::send_value_to (
        $chan,
        $val,
        $cont
      )
    })
  }
}

#[macro_export]
macro_rules! receive_value {
  ( $var:ident => $body:expr ) => {
    $crate::receive_value (
      move | $var | async move {
        $body
      }
    )
  };
  ( ($var:ident $( : $type:ty )?) => $body:expr ) => {
    $crate::receive_value (
      move | $var $( : $type )* | async move {
        $body
      }
    )
  }
}


#[macro_export]
macro_rules! receive_value_from {
  ( $chan:expr,
    $var:ident => $body:expr
  ) => {
    $crate::receive_value_from (
      $chan,
      move | $var | async move {
        $body
      }
    )
  };
  ( $chan:expr,
    ($var:ident $( : $type:ty )?) => $body:expr
  ) => {
    $crate::receive_value_from (
      $chan,
      move | $var $( : $type )* | async move {
        $body
      }
    )
  }
}

#[macro_export]
macro_rules! choose {
  ( $chan:expr,
    $label:ident,
    $cont:expr
  ) => {
    paste::paste! {
      $crate::choose (
        $chan,
        [< $label Label >],
        $cont
      )
    }
  }
}

#[macro_export]
macro_rules! offer_case {
  ( $label:ident,
    $cont:expr
  ) => {
    paste::paste! {
      $crate::offer_case (
        [< $label Label >],
        $cont
      )
    }
  }
}

#[macro_export]
macro_rules! acquire_shared_session {
  ( $chan:expr,
    $var:ident => $body:expr
  ) => {
    $crate::acquire_shared_session (
      $chan.clone(),
      move | $var | async move {
        $body
      }
    )
  }
}

#[macro_export]
macro_rules! receive_channel {
  ( $var:ident => $body:expr ) => {
    $crate::receive_channel (
      move | $var | async move {
        $body
      }
    )
  }
}

#[macro_export]
macro_rules! receive_channels {
  ( ( $var:ident $(,)? ) => $body:expr ) => {
    receive_channel!( $var => $body )
  };
  ( ( $var:ident, $( $vars:ident ),* $(,)? )
    => $body:expr
  ) => {
    receive_channel! ( $var => {
      receive_channels! (
        ( $( $vars ),* ) =>
          $body
      )
    })
  };
}

#[macro_export]
macro_rules! receive_channel_from {
  ( $chan:expr, $var:ident => $body:expr ) => {
    $crate::receive_channel_from (
      $chan,
      move | $var | {
        $body
      }
    )
  }
}

#[macro_export]
macro_rules! include_session {
  ( $session:expr,
    $var:ident => $body:expr
  ) => {
    $crate::include_session (
      $session,
      move | $var | async move {
        $body
      }
    )
  }
}

#[macro_export]
macro_rules! terminate {
  () => {
    $crate::terminate()
  };
  ( $cont:expr ) => {
    $crate::terminate_async(
      move || async move {
        $cont
      }
    )
  };
}

#[macro_export]
macro_rules! wait {
  ( $chan:expr, $cont:expr ) => {
    $crate::wait_async (
      $chan,
      move || async move {
        $cont
      }
    )
  };
}

#[macro_export]
macro_rules! wait_all {
  ( [ $chan:expr $(,)? ],
    $cont:expr
  ) => {
    wait! ( $chan, $cont )
  };
  ( [ $chan:expr, $( $chans:expr ),* $(,)? ],
    $cont:expr
  ) => {
    wait! ( $chan,
      wait_all! (
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
    < HList![ $( $labels ),* ]
      as $crate::Cut < _ >
    > :: cut (
      $cont1,
      move | $var | async move {
        $cont2
      }
    )
  }
}