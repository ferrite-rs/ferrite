use std::time::Duration;
use std::future::{ Future };
use async_std::task::sleep;

use ferrite::*;

type Queue < A > =
  Fix <
    InternalChoice <
      Either <
        End,
        SendValue < A, Z >,
      >
    >
  >;

type StringQueue = Queue < String >;

fn nil_queue < A > ()
  -> Session < Queue < A > >
where
  A : Send + 'static
{
  fix_session (
    offer_case ( LeftLabel,
      terminate ()
    ) )
}

fn append_queue
  < A, Func, Fut >
  ( builder : Func,
    rest : Session < Queue < A > >
  ) ->
    Session < Queue < A > >
where
  A : Send + 'static,
  Func :
    FnOnce() -> Fut
    + Send + 'static,
  Fut:
    Future <
      Output = A
    >
    + Send + 'static
{
  fix_session (
    offer_case ( RightLabel,
      send_value! (
        builder().await,
        rest
      ) ) )
}

fn read_queue () ->
  Session <
    ReceiveChannel <
      StringQueue,
      End
    > >
{
  receive_channel! ( queue => {
    unfix_session_for ( queue,
      case! { queue ;
        Left => {
          wait ( queue, terminate () )
        }
        Right => {
          receive_value_from! ( queue,
            val => {
              println!("Receive value: {}", val);

              include_session! (
                read_queue (),
                next => {
                  send_channel_to (
                    next,
                    queue,
                    forward ( next )
                  ) })
            } )
        }
      }
    )
  })
}

pub fn queue_session () ->
  Session < End >
{
  let p11
    : Session < StringQueue >
  = nil_queue ();

  let p12
    : Session < StringQueue >
  = append_queue ( || async {
      println!("producing world..");
      sleep(Duration::from_secs(3)).await;
      "World".to_string()
    },
    p11
  );

  let p13
    : Session < StringQueue >
  = append_queue ( || async {
      println!("producing hello..");
      sleep(Duration::from_secs(2)).await;
      "Hello".to_string()
    },
    p12
  );

  apply_channel ( read_queue (), p13 )
}

#[async_std::main]
pub async fn main() {
  run_session ( queue_session() ).await;
}
