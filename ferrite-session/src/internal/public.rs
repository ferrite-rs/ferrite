pub mod prelude {
  #[doc(inline)]
  pub use crate::{
    internal::{
      base::public::*,
      functional::*,
      protocol::{
        either,
        public::*,
      },
      session::public::*,
    }
  };

  #[doc(inline)]
  pub use crate::{
    Sum,
    HList,
    offer_choice,
    case,
    define_choice,
    send_value,
    send_value_to,
    receive_value,
    receive_value_from,
    choose,
    offer_case,
    acquire_shared_session,
    receive_channel,
    receive_channels,
    receive_channel_from,
    include_session,
    terminate,
    wait,
    wait_all,
    cut,
  };
}

#[doc(inline)]
pub use crate::internal::{
  base::public as base,
  functional,
  protocol::public as protocol,
  session::public as session,
  protocol::either as either,
};
