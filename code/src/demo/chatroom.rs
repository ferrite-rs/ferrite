// extern crate log;

// use crate::fix::*;
// use crate::base::*;
// use crate::session::*;
// use crate::process::*;

// use std::format;
// use std::thread;
// use std::time::Duration;
// use std::collections::HashMap;

// type IncomingF < R > =
//   ExternalChoice <
//     ReceiveValue <
//       String,
//       R
//     >,
//     End
//   >;

// type OutgoingF < R > =
//   ExternalChoice <
//     SendValue <
//       String,
//       R
//     >,
//     End
//   >;

// struct IncomingT {}
// struct OutgoingT {}

// impl < R > AlgebraT < R > for IncomingT
// where
//   R : Protocol
// {
//   type Algebra = IncomingF < R >;
// }

// impl < R > AlgebraT < R > for OutgoingT
// where
//   R : Protocol
// {
//   type Algebra = OutgoingF < R >;
// }

// type IncomingSession = FixProtocol < IncomingT >;
// type OutgoingSession = FixProtocol < OutgoingT >;

// type ChatRoomSession =
//   ReceiveValue <
//     (String, String),
//     InternalChoice <
//       SendChannel <
//         IncomingSession,
//         OutgoingSession
//       >,
//       End
//     >
//   >;

// fn create_chatroom_server (
//   users : HashMap < String, String >
// ) ->
//   Box < dyn Fn () -> Session < ChatRoomSession > >
// {
//   unimplemented!()
// }

// #[allow(dead_code)]
// pub fn chatroom_session()
//   -> Session < End >
// {
//   unimplemented!()
// }
