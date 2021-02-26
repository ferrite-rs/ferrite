use ferrite_session::prelude::*;
use ipc_channel::ipc;
use log::debug;

// use rand::prelude::*;
// use std::time::Duration;
// use tokio::time::sleep;

type SharedCounter = LinearToShared<SendValue<u64, Z>>;

// async fn random_sleep(start: u64, end: u64) {
//   let sleep_time = thread_rng().gen_range(start, end);
//   sleep( Duration::from_millis ( sleep_time ) ).await;
// }

pub fn make_counter_session(count : u64) -> SharedSession<SharedCounter>
{
  accept_shared_session(move || {
    send_value!(
      {
        debug!("[Server] Producing count {}", count);

        // random_sleep(10, 20).await;
        debug!("[Server] Produced count {}", count);

        count
      },
      detach_shared_session(make_counter_session(count + 1))
    )
  })
}

pub fn read_counter_session(
  name : String,
  stop_at : u64,
  shared : SharedChannel<SharedCounter>,
) -> Session<End>
{
  let shared2 = shared.clone();

  step(async move {
    // random_sleep(10, 20).await;

    acquire_shared_session! ( shared, counter => {
      receive_value_from! ( counter, count => {
        debug!("[{}] Received count: {}", name, count);

        release_shared_session ( counter, {
          if stop_at <= count {
            debug!("[{}] terminating", name);
            terminate()
          } else {
            partial_session (
              read_counter_session ( name, stop_at, shared2 ) )
          }
        })
      })
    })
  })
}

pub fn read_counter_session_2(
  shared_counter : &SharedChannel<SharedCounter>
) -> Session<End>
{
  acquire_shared_session! ( shared_counter, linear_counter => {
    // random_sleep(10, 20).await;
    receive_value_from! ( linear_counter, count => {
      debug!("Received count: {}", count);
      release_shared_session ( linear_counter,
        terminate() )
    })
  })
}

pub fn shared_counter_session() -> Session<End>
{
  let shared = run_shared_session(make_counter_session(0));

  let (sender, receiver) = ipc::channel().unwrap();

  sender.send(shared).unwrap();

  let shared2 = receiver.recv().unwrap();

  // let shared2 = shared.clone();

  let mut sessions = vec![];

  for i in 0..10000 {
    sessions.push(read_counter_session(format!("P{}", i), 10, shared2.clone()));
  }

  wait_sessions(sessions, terminate())
}

#[tokio::main]

pub async fn main()
{
  env_logger::init();

  run_session(shared_counter_session()).await;
}
