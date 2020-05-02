#![feature(type_ascription)]
#![feature(async_closure)]
#[macro_use]

extern crate log;
// extern crate simple_logger;
extern crate env_logger;

pub mod demo;
pub mod macros;

pub mod base;
pub mod shared;
pub mod session;
pub mod process;
pub mod processes;

mod public;

pub use crate::demo::*;
pub use crate::public::*;

#[async_std::main]
pub async fn main() {
  // simple_logger::init().unwrap();
  env_logger::init();

  info!("[Main] Running main program");

  // run_session(hello_session()).await;
  // run_session(pair_session()).await;
  // run_session(restaurant_session()).await;
  // run_session(concat_session()).await;
  // run_session(queue_session()).await;
  // run_session(counter_session()).await;
  run_session ( shared_counter_session() ).await;

  // run_session (
  //   demo::channel::channel_session()
  // ).await;

  // run_session (
  //   demo::nary_choice::nary_choice_demo()
  // ).await;

  // run_session (
  //   demo::queue::queue_session()
  // ).await;

  // run_session (
  //   demo::stream::stream_session()
  // ).await;

  // run_session (
  //   demo::stream2::stream_session()
  // ).await;

  info!("[Main] Main program terminating");
}
