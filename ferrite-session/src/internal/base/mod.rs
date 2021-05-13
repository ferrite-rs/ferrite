mod channel;
mod context;
mod protocol;
mod rec;
mod session;
mod shared;

pub mod public;

#[doc(inline)]
pub use self::{
  channel::{
    ipc_channel,
    once_channel,
    opaque_channel,
    unbounded,
    ForwardChannel,
    IpcReceiver,
    IpcSender,
    OpaqueReceiver,
    OpaqueSender,
    Receiver,
    ReceiverF,
    ReceiverOnce,
    Sender,
    SenderF,
    SenderOnce,
    Value,
  },
  context::{
    AppendContext,
    Context,
    ContextLens,
    Empty,
    EmptyContext,
    Slot,
  },
  protocol::{
    Protocol,
    SharedProtocol,
  },
  rec::{
    fix,
    unfix,
    Fixed,
    HasRecApp,
    Rec,
    RecApp,
    RecX,
    Release,
    SharedRecApp,
  },
  session::{
    unsafe_create_session,
    unsafe_run_session,
    PartialSession,
    Session,
  },
  shared::{
    unsafe_create_shared_channel,
    unsafe_create_shared_session,
    unsafe_receive_shared_channel,
    unsafe_run_shared_session,
    SharedChannel,
    SharedSession,
  },
};
