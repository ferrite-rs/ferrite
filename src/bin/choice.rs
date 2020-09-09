#![feature(async_closure)]

use ferrite::*;
use ferrite::choice::binary::*;
use ferrite::choice::nary::{
  run_external_cont,
  run_internal_cont,
};
use ferrite::choice::nary::either as either;

use std::time::Duration;
use async_std::task::sleep;

pub struct MushroomSoup {}
pub struct TomatoSoup {}
pub struct BeefSteak {}
pub struct PorkChop {}

pub fn restaurant_session()
  -> Session < End >
{
  /*
                cleanup = terminate_async () :: · ⊢ End
    ========================================================
        builder1() = (MushroomSoup, cleanup :: · ⊢ End)
    ========================================================
              cont1 = send_value_async(builder1)
                :: · ⊢ (MushroomSoup ∧ End)
    ========================================================
      soup_of_the_day = offer_left(cont1)
        :: · ⊢ (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
   */
  let soup_of_the_day :
    Session <
      InternalChoice <
        SendValue < MushroomSoup, End >,
        SendValue < TomatoSoup, End >
      >
    >
  = offer_left(
      send_value_async ( async || {
        println!("[Soup] Spending 3 seconds to prepare mushroom soup");
        sleep(Duration::from_secs(2)).await;
        println!("[Soup] Finished preparing mushroom soup");

        ( MushroomSoup {}
        , terminate_async ( async || {
            println!("[Soup] Served mushroom soup. Terminating soup protocol");
          })
        )
      }));

  /*

            cleanup1, cleanup2 = terminate_async () :: · ⊢ End
    ===============================================================
      builder1()                      builder2()
        = (BeefSteak,                   = (PorkChop,
           cleanup1 :: · ⊢ End)           cleanup2:: · ⊢ End)
    ================================================================
      main_dish_cont(Left)            main_dish_cont(Right)
        = send_value_async(builder1)        =  send_value_async(builder2)
        :: · ⊢ BeefSteak ∧ End         :: · ⊢ PorkChop ∧ End
    ================================================================
      main_dish = offer_choice(main_dish_cont)
        :: · ⊢ (BeefSteak ∧ End) & (PorkChop ∧ End)
   */

  let main_dish
    : Session <
        ExternalChoice <
          SendValue < BeefSteak, End >,
          SendValue < PorkChop, End >
        >
      >
  = offer_choice(| option | {
      match either::extract(option) {
        either::Left ( cont ) => {
          println!("[MainCourse] Customer chose to eat beef steak");

          run_external_cont ( cont,
            send_value_async( async || {
              println!("[MainCourse] Spending 7 seconds to prepare beef steak");
              sleep(Duration::from_secs(7)).await;

              ( BeefSteak{}

              , terminate_async ( async || {
                  println!("[MainCourse] Served beef steak. Terminating main course protocol");
                })
              )
            }))
        }
        either::Right (cont) => {
          println!("[MainCourse] Customer chose to eat pork chop");

          run_external_cont ( cont,
            send_value_async ( async || {
              println!("[MainCourse] Spending 5 seconds to prepare pork chop");
              sleep(Duration::from_secs(5)).await;

              ( PorkChop{}

              , terminate_async ( async || {
                  println!("[MainCourse] Served pork chop. Terminating main course protocol");
                })
              )
            }))
        }
      }
    });

  /*
      main_dish :: · ⊢ (BeefSteak ∧ End) & (PorkChop ∧ End)
    ============================================================
      send_main_dish
        = send_channel_from (main_dish)
        :: (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
              ⊢ (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
                 ⊗ ((BeefSteak ∧ End) & (PorkChop ∧ End))

      soup_of_the_day
        :: · ⊢ (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
    =============================================================
      menu = link(soup_of_the_day, send_main_dish)
        :: · ⊢ (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
                ⊗ ((BeefSteak ∧ End) & (PorkChop ∧ End))

   */
  let menu : Session <
    SendChannel <
      InternalChoice<
        SendValue < MushroomSoup, End >,
        SendValue < TomatoSoup, End >
      >,
      ExternalChoice <
        SendValue < BeefSteak, End >,
        SendValue < PorkChop, End >
      >
    >
  > =
    include_session ( soup_of_the_day, | chan | {
      send_channel_from ( chan,
        partial_session( main_dish ) )
    });

  /*
      cont4 = choose_right(cont6)                     cont5 = choose_left(cont7)
        :: (BeefSteak ∧ End) & (PorkChop ∧ End)       :: (BeefSteak ∧ End) & (PorkChop ∧ End)
            ⊢ End                                          ⊢ End
    =================================================================================================
      cont2(MushroomSoup)                             cont3(TomatoSoup)
        = wait_async (cont4)                                   wait_async (cont5)
        :: End,                                        :: End,
           ((BeefSteak ∧ End) & (PorkChop ∧ End))        ((BeefSteak ∧ End) & (PorkChop ∧ End))
            ⊢ End                                          ⊢ End
    ==================================================================================================
      diner_cont(Left)                                diner_cont(Right)
        = receive_value_from(cont2)                         = receive_value_from(cont3)
        :: (MushroomSoup ∧ End),                       :: (MushroomSoup ∧ End),
           ((BeefSteak ∧ End) & (PorkChop ∧ End))         ((BeefSteak ∧ End) & (PorkChop ∧ End))
            ⊢ End                                            ⊢ End
    ===================================================================================================
      cont1 = case(diner_cont)
        :: (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End),
           ((BeefSteak ∧ End) & (PorkChop ∧ End))
            ⊢ End
    ====================================================================================================
      cont0 = receive_channel_from(cont1)
        :: (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
            ⊗ ((BeefSteak ∧ End) & (PorkChop ∧ End))
           ⊢ End
    ====================================================================================================
      diner = receive_channel(cont0)
        :: ⊢ ( (MushroomSoup ∧ End) ⊕ (TomatoSoup ∧ End)
               ⊗ ((BeefSteak ∧ End) & (PorkChop ∧ End))
             )
             ⊸ End
   */
  let diner
    : Session <
        ReceiveChannel <
          SendChannel <
            InternalChoice <
              SendValue < MushroomSoup, End >,
              SendValue < TomatoSoup, End >
            >,
            ExternalChoice <
              SendValue < BeefSteak, End >,
              SendValue < PorkChop, End >
            >
          >,
          End
        >
      >
  = receive_channel ( move | menu_chan | {
      receive_channel_from ( menu_chan, move | soup_chan | {
        case( soup_chan, move | option | {
          match either::extract (option) {
            either::Left ( cont ) => {
              println!("[Diner] Restaurant offers mushroom soup today");
              run_internal_cont ( cont,
                receive_value_from( soup_chan, async move | _mushroom_soup | {
                  println!("[Diner] Received mushroom soup. Spending 2 seconds drinking it");
                  sleep(Duration::from_secs(2)).await;
                  println!("[Diner] Finished drinking mushroom soup");

                  println!("[Diner] Choosing pork chop for main");
                  wait_async ( soup_chan, async move || {
                    println!("[Diner] Soup protocol terminated");

                    choose_right( menu_chan,
                      receive_value_from( menu_chan, async move | _pork_chop | {
                        println!("[Diner] Received pork chop. Spending 5 seconds eating it");
                        sleep(Duration::from_secs(5)).await;
                        println!("[Diner] Finished eating pork chop");

                        wait_async ( menu_chan, async || {
                          println!("[Diner] Main course protocol terminated");

                          terminate_async ( async || {
                            println!("[Diner] Spending 4 seconds in washroom");
                            sleep(Duration::from_secs(4)).await;
                            println!("[Diner] Leaving restaurant");
                          })
                        })
                      }))
                  })
                }))
            }
            either::Right (cont) => {
              println!("[Diner] Restaurant offers tomato soup today");
              run_internal_cont ( cont,
                receive_value_from( soup_chan, async move | _tomato_soup | {
                  println!("[Diner] Received tomato soup. Spending 1 second drinking it");

                  sleep(Duration::from_secs(1)).await;

                  println!("[Diner] finished drinking tomato soup");
                  println!("[Diner] Choosing beef steak for main");

                  wait_async ( soup_chan, async move || {
                    println!("[Diner] Soup protocol terminated");

                    choose_left( menu_chan,
                      receive_value_from( menu_chan, async move | _beef_steak | {
                        println!("[Diner] Received beef steak. Spending 6 seconds eating it");
                        sleep(Duration::from_secs(6)).await;
                        println!("[Diner] Finished eating beef steak.");

                        wait_async ( menu_chan, async || {
                          println!("[Diner] Main course protocol terminated");

                          terminate_async ( async || {
                            println!("[Diner] Spending 3 seconds in washroom");
                            sleep(Duration::from_secs(3)).await;
                            println!("[Diner] Leaving restaurant");
                          })
                        })
                      }))
                  })
                }))
            }
          }
        })
      })
    });

  let restaurant = apply_channel(diner, menu);

  return restaurant;
}


#[async_std::main]
pub async fn main() {
  run_session ( restaurant_session () ) .await;
}
