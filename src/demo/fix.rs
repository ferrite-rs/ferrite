extern crate log;
use std::time::Duration;
use async_std::task::sleep;

use ferrite::*;
use ferrite::choice::binary::*;

type Stream =
  Fix <
    ReceiveValue <
      String,
      InternalChoice <
        Z,
        Fix <
          SendValue <
            i64,
            ExternalChoice <
              Z,
              S < Z >
            > > > > > >;

fn stream_server (seed: i64)
  -> Session < Stream >
{
  fix_session (
    receive_value ( async move | password | {
      if password == "secret" {
        offer_left (
          stream_server ( seed ) )
      } else {
        offer_right (
          fix_session (
            send_stream ( seed )
          ) ) } }) )
}

fn send_stream ( seed : i64 ) ->
  Session <
    SendValue <
      i64,
      ExternalChoice <
        Fix <
          SendValue <
            i64,
            ExternalChoice <
              Z,
              S < Stream > > > >,
        Stream > > >
{
  send_value_async ( async move || {
    sleep(Duration::from_secs(1)).await;
    ( seed,
      offer_choice ( move | choice | {
        match choice {
          Either::Left (ret) => {
            ret ( fix_session (
              send_stream ( seed + 1 )
            ) )
          },
          Either::Right (ret) => {
            ret ( stream_server ( seed + 1 ) )
          }, } }) ) } )
}

pub fn nested_fix_session () ->
  Session < End >
{
  let _p1 : Session < Stream > =
    stream_server ( 0 );

  unimplemented!()
}
