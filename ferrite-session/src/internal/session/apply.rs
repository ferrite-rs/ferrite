use crate::internal::{
  base::{
    Protocol,
    Session,
  },
  protocol::ReceiveChannel,
  session::{
    channel::send_channel_to,
    forward::forward,
    include::include_session,
  },
};

pub fn apply_channel<A, B>(
  f : Session<ReceiveChannel<A, B>>,
  a : Session<A>,
) -> Session<B>
where
  A : Protocol,
  B : Protocol,
{
  include_session(f, move |c1| {
    include_session(a, move |c2| send_channel_to(c1, c2, forward(c1)))
  })
}
