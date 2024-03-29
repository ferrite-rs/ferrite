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
    ReceiverOnce,
    Sender,
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
    ClientEndpoint,
    ClientEndpointF,
    Protocol,
    ProviderEndpoint,
    ProviderEndpointF,
    SealedProtocol,
    SealedSharedProtocol,
    SharedProtocol,
  },
  rec::{
    fix,
    unfix,
    HasRecApp,
    Rec,
    RecApp,
    RecEndpoint,
    RecRow,
    RecX,
    Release,
    SharedRecApp,
    SharedRecRow,
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
    unsafe_forward_shared_channel,
    unsafe_receive_shared_channel,
    unsafe_run_shared_session,
    SharedChannel,
    SharedSession,
  },
};
