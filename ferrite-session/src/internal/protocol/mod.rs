pub mod public;

mod channel;
mod choice;
mod end;

mod shared;
mod value;
mod wrap;

#[doc(inline)]
pub use self::{
  channel::{
    ReceiveChannel,
    SendChannel,
  },
  choice::{
    either,
    ExternalChoice,
    InternalChoice,
  },
  end::End,
  shared::{
    LinearToShared,
    Lock,
    SharedToLinear,
  },
  value::{
    ReceiveValue,
    SendValue,
  },
  wrap::{
    Wrap,
    Wrapper,
  },
};
