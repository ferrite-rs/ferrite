extern crate log;

use std::time::Duration;
use std::collections::VecDeque;
use std::pin::Pin;
use std::future::Future;
use async_std::task::sleep;

use crate::public::*;

pub type Receiver < T > =
  FixProcess <
    ExternalChoice <
      SendValue <
        T,
        Recurse
      >,
      End
    >
  >;

pub type Channel < T > =
  LinearToShared <
    ExternalChoice <
      ReceiveValue < T, Release >,
      SendValue <
        Option < T >,
        Release
      >
    >
  >;

pub fn make_receiver
  < T >
  ( source :
      SharedSession <
        Channel < T >
      >
  ) ->
    Session <
      Receiver < T >
    >
where
  T : Send + 'static
{
  acquire_shared_session ( source.clone(), move | chan | {
    choose_right ( chan,
      receive_value_from ( chan, async move | m_val | {
        match m_val {
          Some ( val ) => {
            fix_session (
              offer_choice ( move | option | {
                match option {
                  Either::Left ( ret ) => {
                    ret (
                      send_value ( val,
                        release_shared_session ( chan,
                          fill_hole (
                            partial_session (
                              make_receiver ( source ) )
                          ) ) ) )
                  },
                  Either::Right ( ret ) => {
                    ret (
                      release_shared_session ( chan,
                        terminate () ) )
                  }
                }
              }) )
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
      SharedSession <
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
  acquire_shared_session ( source, move | chan | {
    choose_left ( chan,
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
    SuspendedSharedSession <
      Channel < T >
    >
where
  T : Send + 'static
{
  accept_shared_session(
    offer_choice ( | option | {
      match option {
        Either::Left ( ret ) => {
          ret (
            receive_value ( async move | val | {
              queue.push_back ( val );
              detach_shared_session (
                do_create_channel ( queue ) )
            }) )
        },
        Either::Right (ret ) => {
          let m_val = queue.pop_front();

          ret (
            send_value ( m_val,
              detach_shared_session (
                do_create_channel ( queue ) ) ) )
        }
      }
    })
  )
}

pub fn create_channel
  < T >
  () ->
    SharedSession <
      Channel < T >
    >
where
  T : Send + 'static
{
  run_shared_session (
    do_create_channel(
      VecDeque::new() ) )
}

pub fn channel_session ()
  -> Session < End >
{
  let channel :
    SharedSession <
      Channel < String >
    > =
      create_channel ();

  let consumer1 : Session < End > =
    include_session (
      make_receiver ( channel.clone() ),
      | receiver | {
        unfix_session ( receiver,
          choose_left ( receiver,
            receive_value_from ( receiver, async move | val | {
              info!("[Consumer 1] Receive first value: {}", val);

              unfix_hole ( receiver,
                choose_right ( receiver,
                  wait ( receiver,
                    terminate () ) ) )
            } ) ))
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

  wait_sessions (
    vec! [ consumer1, producer1, producer2 ],
    terminate () )
}