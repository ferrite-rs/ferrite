#![feature(async_closure)]

use std::time::Duration;
use std::collections::VecDeque;
use std::pin::Pin;
use std::future::Future;
use async_std::task::sleep;

use ferrite::*;

// Example implementation of Rust channels using shared channels

define_choice! {
  ReceiverOption < T > ;
  Next: SendValue < T, Z >,
  Close: End,
}

define_choice! {
  ChannelOption < T > ;
  ReceiveNext: ReceiveValue < T, Z >,
  SendNext: SendValue <
    Option < T >,
    Z
  >,
}

pub type Receiver < T > =
  Fix <
    ExternalChoice <
      ReceiverOption < T >
    >
  >;

pub type Channel < T > =
  LinearToShared <
    ExternalChoice <
      ChannelOption < T >
    >
  >;

pub fn make_receiver
  < T >
  ( source :
      SharedChannel <
        Channel < T >
      >
  ) ->
    Session <
      Receiver < T >
    >
where
  T : Send + 'static
{
  acquire_shared_session ( source.clone(), async move | chan | {
    choose ( chan, SendNextLabel,
      receive_value_from ( chan, async move | m_val | {
        match m_val {
          Some ( val ) => {
            fix_session (
              offer_choice! {
                Next => {
                  send_value ( val,
                    release_shared_session ( chan,
                        partial_session (
                          make_receiver ( source ) ) )
                      )
                }
                Close => {
                  release_shared_session ( chan,
                    terminate () )
                }
              })
          },
          None => {
            sleep(Duration::from_millis(100)).await;

            release_shared_session ( chan,
              partial_session (
                make_receiver ( source ) ) )
          }
        }
      }) )
  })
}

pub fn sender_session
  < T, Func >
  ( source :
      SharedChannel <
        Channel < T >
      >,
    make_val : Func
  ) ->
    Session < End >
where
  T : Send + 'static,
  Func :
    FnOnce () ->
      Pin < Box <
          dyn Future < Output = T > + Send
      > >
      + Send + 'static,
{
  acquire_shared_session ( source, async move | chan | {
    choose ( chan, ReceiveNextLabel,
      send_value_to_async ( chan, async move || {
        let val = make_val().await;

        ( val,
          release_shared_session ( chan,
            terminate () )
        )
      }) )
  })
}

fn do_create_channel
  < T >
  ( mut queue :
      VecDeque < T >
  ) ->
    SharedSession <
      Channel < T >
    >
where
  T : Send + 'static
{
  accept_shared_session (
    offer_choice! {
      ReceiveNext => {
        receive_value ( async move | val | {
          queue.push_back ( val );
          detach_shared_session (
            do_create_channel ( queue ) )
        })
      }
      SendNext => {
        let m_val = queue.pop_front();

        send_value ( m_val,
          detach_shared_session (
            do_create_channel ( queue ) ) )
      }
    })
}

pub fn create_channel
  < T >
  () ->
    SharedChannel <
      Channel < T >
    >
where
  T : Send + 'static
{
  let (session, _) =
    run_shared_session (
      do_create_channel(
        VecDeque::new() ) );

  session
}

pub fn channel_session ()
  -> Session < End >
{
  let channel :
    SharedChannel <
      Channel < String >
    > =
      create_channel ();

  let consumer1 : Session < End > =
    include_session (
      make_receiver ( channel.clone() ),
      | receiver | {
        unfix_session_for ( receiver,
          choose ( receiver, NextLabel,
            receive_value_from ( receiver, async move | val | {
              println!("[Consumer 1] Receive first value: {}", val);

              unfix_session_for ( receiver,
                choose ( receiver, NextLabel,
                  receive_value_from ( receiver, async move | val | {
                    println!("[Consumer 1] Receive second value: {}", val);

                    unfix_session_for ( receiver,
                      choose ( receiver, CloseLabel,
                        wait ( receiver,
                          terminate () ) )
                    )
                  } ) )
              )
            } ) )
        )
      });

  let producer1 : Session < End > =
    sender_session ( channel.clone(), || {
      Box::pin ( async {
        sleep(Duration::from_secs(2)).await;
        "hello".to_string()
      })
    });

  let producer2 : Session < End > =
    sender_session ( channel.clone(), || {
      Box::pin ( async {
        sleep(Duration::from_secs(1)).await;
        "world".to_string()
      })
    });

  let producer3 : Session < End > =
    sender_session ( channel.clone(), || {
      Box::pin ( async {
        sleep(Duration::from_secs(3)).await;
        "bye".to_string()
      })
    });

  wait_sessions (
    vec! [ consumer1, producer1, producer3, producer2 ],
    terminate () )
}

#[async_std::main]
pub async fn main() {
  run_session( channel_session() ) .await;
}
