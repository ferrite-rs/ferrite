use std::time::Duration;

use ferrite_session::prelude::*;
// use ipc_channel::ipc;
use rand::prelude::*;
use tokio::time::sleep;

type SharedCounter = LinearToShared<SendValue<u64, Release>>;

async fn random_sleep()
{
  let sleep_time = thread_rng().gen_range(50, 100);
  sleep(Duration::from_millis(sleep_time)).await;
}

pub fn make_counter_session(count: u64) -> SharedSession<SharedCounter>
{
  accept_shared_session(step(async move {
    println!("[Server] Producing count {}", count);
    random_sleep().await;
    println!("[Server] Produced count {}", count);

    send_value(
      count,
      detach_shared_session(make_counter_session(count + 1)),
    )
  }))
}

pub fn read_counter_session(
  name: String,
  stop_at: u64,
  shared: SharedChannel<SharedCounter>,
) -> Session<End>
{
  let shared2 = shared.clone();

  step(async move {
    random_sleep().await;

    acquire_shared_session(shared, move |counter| {
      receive_value_from(counter, move |count| {
        println!("[{}] Received count: {}", name, count);

        release_shared_session(counter, {
          if stop_at <= count {
            println!("[{}] terminating", name);
            terminate()
          } else {
            partial_session(read_counter_session(name, stop_at, shared2))
          }
        })
      })
    })
  })
}

pub fn read_counter_session_2(
  shared_counter: &SharedChannel<SharedCounter>
) -> Session<End>
{
  acquire_shared_session(shared_counter.clone(), move |linear_counter| {
    step(async move {
      random_sleep().await;
      receive_value_from(linear_counter, move |count| {
        println!("Received count: {}", count);
        release_shared_session(linear_counter, terminate())
      })
    })
  })
}

pub fn shared_counter_session() -> Session<End>
{
  let shared = run_shared_session(make_counter_session(0));

  // Sending a shared channel through IPC channel causes it
  // to be serialized and deserialized through OS socket.
  // let (sender, receiver) = ipc::channel().unwrap();
  // sender.send(shared).unwrap();
  // let shared = receiver.recv().unwrap();

  let mut sessions = vec![];

  for i in 0..100 {
    sessions.push(read_counter_session(format!("P{}", i), 10, shared.clone()));
  }

  wait_sessions(sessions, terminate())
}

#[tokio::main]
pub async fn main()
{
  env_logger::init();

  run_session(shared_counter_session()).await;
}
