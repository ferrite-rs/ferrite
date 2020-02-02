extern crate log;

use crate::public::*;
use crate::processes::lens::*;
use crate::public::nary_choice as nary;
use nary::{ Sum };

struct Foo {}
struct Bar {}
struct Baz {}

type SendFoo = SendValue < Foo, End >;
type SendBar = SendValue < Bar, End >;
type SendBaz = SendValue < Baz, End >;

type SendFooBarBaz =
  Sum <
    SendFoo,
    Sum <
      SendBar,
      SendBaz
    >
  >;

pub fn nary_choice_demo ()
  -> Session < End >
{
  let p1
    : Session <
        nary::ExternalChoice <
          SendFooBarBaz
        >
      > =
    nary::offer_choice :: < _, SendFooBarBaz, _ > (
      | choice
      | {
        match choice {
          Sum::Inl (ret_foo) => {
            info!("[p1] sending foo");
            ret_foo (
              send_value ( Foo {},
                terminate () ) )
          },
          Sum::Inr (barbaz) => {
            match barbaz {
              Sum::Inl (ret_bar) => {
                info!("[p1] sending bar");
                ret_bar (
                  send_value ( Bar {},
                    terminate () ) )
              },
              Sum::Inr (ret_baz) => {
                info!("[p1] sending baz");
                ret_baz (
                  send_value ( Baz {},
                    terminate () ) )
              }
            }
          }
        }
      });

  let p2 : Session < End > =
    include_session ( p1,
      move | chan | {
        nary::choose (
          chan, select_2(),
          receive_value_from (
            chan, async move | _ : Baz | {
              info!("[p2] received baz");
              wait ( chan,
                terminate() )
            })
        )
      });

  let p3 :
    Session <
      nary::InternalChoice <
        SendFooBarBaz
      >
    > =
    nary::offer_case ( select_2(),
      send_value ( Baz {},
        terminate () ) );

  let p4 :
    Session < End > =
    include_session ( p3,
      | chan | {
        nary::case::< _, _, _, SendFooBarBaz, _ > ( chan,
          move | branch | {
            match branch {
              Sum::Inl (ret_foo) => {
                ret_foo (
                  receive_value_from ( chan,
                    async move | _ : Foo | {
                      info!("[p4] received foo");
                      wait ( chan,
                        terminate () )
                    }) )
              },
              Sum::Inr (branch2) => {
                match branch2 {
                  Sum::Inl (ret_bar) => {
                    ret_bar (
                      receive_value_from ( chan,
                        async move | _ : Bar | {
                          info!("[p4] received bar");
                          wait ( chan,
                            terminate () )
                        }) )
                  },
                  Sum::Inr (ret_baz) => {
                    ret_baz (
                      receive_value_from ( chan,
                        async move | _ : Baz | {
                          info!("[p4] received baz");
                          wait ( chan,
                            terminate () )
                        }) )
                  }
                }
              }
            }
          })
      });

  wait_sessions (
    vec! [ p2, p4 ],
    terminate () )
}