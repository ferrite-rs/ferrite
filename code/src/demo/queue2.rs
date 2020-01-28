use async_macros::join;

use crate::base::*;
use crate::process::fix2::*;
use crate::session::fix2::*;

use crate::process::{
  InternalChoice,
  SendChannel,
  SendValue,
  End
};

use crate::session::{
  offer_left,
  offer_right,
  terminate,
  send_value,
};

type StringQueue =
  FixProcess <
    InternalChoice <
      End,
      SendValue < String, Zero >,
    >
  >;

fn nil_queue ()
  -> Session < StringQueue >
{
  fix_session (
    offer_left (
      terminate ()
    )
  )
}

fn one_queue ()
  -> Session < StringQueue >
{
  fix_session (
    offer_right (
      send_value (
        "one".to_string(),
        nil_queue ()
      ) ) )
}

fn two_queue ()
  -> Session < StringQueue >
{
  fix_session (
    offer_right (
      send_value (
        "two".to_string(),
        one_queue ()
      ) ) )
}
