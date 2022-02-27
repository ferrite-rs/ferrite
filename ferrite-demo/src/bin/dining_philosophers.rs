/*
 Exercise: Dining Philosophers

 In this exercise we will implement the dining philosophers
 problem using session type.

   - Implement the fork as shared process.

     - Each fork is identified by a unique ID represented as u8.

     - When the fork is acquired, prints
       "fork {id} has been acquired"

   - Implement the philosopher as a linear process.

     - Each philosopher is identified by a unique ID represented as u8

     - Each philosopher is given two shared channels, representing the
       left and right forks.

     - The philosopher starts thinking by printing
       "philosopher {id} is thinking",
       then pauses for 1 second.

     - After finished thinking, print
       "philosopher {id} is going to eat"

     - Try to acquire the left fork, then prints
       "philosopher {id} has acquired the left fork"

     - Try to acquire the right fork, then prints
       "philosopher {id} has acquired the right fork"

     - Print "philosopher {id} is eating", then pause for 1 second.

     - After finished eating, print
       "philosopher {} has finished eating and is releasing the forks",

     - Release the right fork, followed by the left fork.

     - Start from the beginning again.
*/

use std::time::Duration;

use ferrite_session::prelude::*;
use tokio::time::sleep;

type Fork = LinearToShared<Release>;

fn fork(id: u8) -> SharedSession<Fork>
{
  accept_shared_session(async move {
    step(async move {
      println!("fork {} has been acquired", id);
      detach_shared_session(fork(id))
    })
  })
}

fn run_fork(id: u8) -> SharedChannel<Fork>
{
  run_shared_session(fork(id))
}

fn philosopher(
  id: u8,
  left: SharedChannel<Fork>,
  right: SharedChannel<Fork>,
) -> Session<End>
{
  step(async move {
    println!("philosopher {} is thinking", id);
    sleep(Duration::from_secs(1)).await;

    println!("philosopher {} is going to eat", id);
    acquire_shared_session(left.clone(), move |left_fork| {
      println!("philosopher {} has acquired the left fork", id);
      acquire_shared_session(right.clone(), move |right_fork| {
        println!("philosopher {} has acquired the right fork", id);
        step(async move {
          println!("philosopher {} is eating", id);
          sleep(Duration::from_secs(1)).await;

          println!(
            "philosopher {} has finished eating and is releasing the forks",
            id
          );

          release_shared_session(
            right_fork,
            release_shared_session(
              left_fork,
              include_session(philosopher(id, left, right), forward),
            ),
          )
        })
      })
    })
  })
}

fn main_session() -> Session<End>
{
  let f0 = run_fork(0);
  let f1 = run_fork(1);
  let f2 = run_fork(2);
  let f3 = run_fork(3);

  let p0 = philosopher(0, f0, f1.clone());
  let p1 = philosopher(1, f1.clone(), f2.clone());
  let p2 = philosopher(2, f2, f3.clone());

  // Using this version of p3 will result in deadlock:
  // let p3 = philosopher(3, f3, f1);

  let p3 = philosopher(3, f1, f3);

  include_session(p0, move |c0| {
    include_session(p1, move |c1| {
      include_session(p2, move |c2| {
        include_session(p3, move |c3| wait_all!([c0, c1, c2, c3], terminate()))
      })
    })
  })
}

#[tokio::main]
pub async fn main()
{
  env_logger::init();

  run_session(main_session()).await;
}
