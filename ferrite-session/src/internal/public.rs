pub mod prelude
{
  #[doc(inline)]
  pub use crate::internal::{
    base::public::*,
    functional::*,
    protocol::{
      either,
      public::*,
    },
    session::public::*,
  };
  #[doc(inline)]
  pub use crate::{
    acquire_shared_session,
    case,
    choose,
    cut,
    define_choice,
    include_session,
    offer_case,
    offer_choice,
    receive_channel,
    receive_channel_from,
    receive_channels,
    receive_value,
    receive_value_from,
    send_value,
    send_value_to,
    terminate,
    wait,
    wait_all,
    HList,
    Sum,
  };
}

#[doc(inline)]
pub use crate::internal::{
  base::public as base,
  functional,
  protocol::either,
  protocol::public as protocol,
  session::public as session,
};
