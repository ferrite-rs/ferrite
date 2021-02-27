pub mod public;

mod channel;
mod choice;
mod end;
mod linear_to_shared;
mod lock;
mod shared_to_linear;
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
  linear_to_shared::LinearToShared,
  lock::Lock,
  shared_to_linear::SharedToLinear,
  value::{
    ReceiveValue,
    SendValue,
  },
  wrap::{
    Wrap,
    Wrapper,
  },
};
