extern crate log;

use crate::public::*;

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
      InternalChoice<
        SendValue < MushroomSoup, End >,
        SendValue < TomatoSoup, End >
      >
    >
  = offer_left(
      send_value_async ( async || {
        info!("[Soup] Spending 3 seconds to prepare mushroom soup");
        sleep(Duration::from_secs(2)).await;
        info!("[Soup] Finished preparing mushroom soup");

        ( MushroomSoup {}
        , terminate_async ( async || {
            info!("[Soup] Served mushroom soup. Terminating soup process");
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
      match option {
      Either::Left(return_left) => {
        info!("[MainCourse] Customer chose to eat beef steak");

        return_left (
          send_value_async( async || {
            info!("[MainCourse] Spending 7 seconds to prepare beef steak");
            sleep(Duration::from_secs(7)).await;

            ( BeefSteak{}

            , terminate_async ( async || {
                info!("[MainCourse] Served beef steak. Terminating main course process");
              })
            )
          }))
      }
      Either::Right(return_right) => {
        info!("[MainCourse] Customer chose to eat pork chop");

        return_right (
          send_value_async ( async || {
            info!("[MainCourse] Spending 5 seconds to prepare pork chop");
            sleep(Duration::from_secs(5)).await;

            ( PorkChop{}

            , terminate_async ( async || {
                info!("[MainCourse] Served pork chop. Terminating main course process");
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
          match option {
          Either::Left(return_left) => {
            info!("[Diner] Restaurant offers mushroom soup today");
            return_left(
              receive_value_from( soup_chan, async move | _mushroom_soup | {
                info!("[Diner] Received mushroom soup. Spending 2 seconds drinking it");
                sleep(Duration::from_secs(2)).await;
                info!("[Diner] Finished drinking mushroom soup");

                info!("[Diner] Choosing pork chop for main");
                wait_async ( soup_chan, async move || {
                  info!("[Diner] Soup process terminated");

                  choose_right( menu_chan,
                    receive_value_from( menu_chan, async move | _pork_chop | {
                      info!("[Diner] Received pork chop. Spending 5 seconds eating it");
                      sleep(Duration::from_secs(5)).await;
                      info!("[Diner] Finished eating pork chop");

                      wait_async ( menu_chan, async || {
                        info!("[Diner] Main course process terminated");

                        terminate_async ( async || {
                          info!("[Diner] Spending 4 seconds in washroom");
                          sleep(Duration::from_secs(4)).await;
                          info!("[Diner] Leaving restaurant");
                        })
                      })
                    }))
                })
              }))
          }
          Either::Right(return_right) => {
            info!("[Diner] Restaurant offers tomato soup today");
            return_right(
              receive_value_from( soup_chan, async move | _tomato_soup | {
                info!("[Diner] Received tomato soup. Spending 1 second drinking it");

                sleep(Duration::from_secs(1)).await;

                info!("[Diner] finished drinking tomato soup");
                info!("[Diner] Choosing beef steak for main");

                wait_async ( soup_chan, async move || {
                  info!("[Diner] Soup process terminated");

                  choose_left( menu_chan,
                    receive_value_from( menu_chan, async move | _beef_steak | {
                      info!("[Diner] Received beef steak. Spending 6 seconds eating it");
                      sleep(Duration::from_secs(6)).await;
                      info!("[Diner] Finished eating beef steak.");

                      wait_async ( menu_chan, async || {
                        info!("[Diner] Main course process terminated");

                        terminate_async ( async || {
                          info!("[Diner] Spending 3 seconds in washroom");
                          sleep(Duration::from_secs(3)).await;
                          info!("[Diner] Leaving restaurant");
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

/*
  Example Log:

  12:23:31 INFO  [process_builder_dynamics] [Main] Running main program
  12:23:31 INFO  [process_builder_dynamics::demo::choice] [Soup] Spending 3 seconds to prepare mushroom soup
  12:23:31 INFO  [process_builder_dynamics::demo::choice] [Diner] Restaurant offers mushroom soup today
  12:23:33 INFO  [process_builder_dynamics::demo::choice] [Soup] Finished preparing mushroom soup
  12:23:33 INFO  [process_builder_dynamics::demo::choice] [Diner] Received mushroom soup. Spending 2 seconds drinking it
  12:23:33 INFO  [process_builder_dynamics::demo::choice] [Soup] Served mushroom soup. Terminating soup process
  12:23:35 INFO  [process_builder_dynamics::demo::choice] [Diner] Finished drinking mushroom soup
  12:23:35 INFO  [process_builder_dynamics::demo::choice] [Diner] Choosing pork chop for main
  12:23:35 INFO  [process_builder_dynamics::demo::choice] [Diner] Soup process terminated
  12:23:35 INFO  [process_builder_dynamics::demo::choice] [MainCourse] Customer chose to eat pork chop
  12:23:35 INFO  [process_builder_dynamics::demo::choice] [MainCourse] Spending 5 seconds to prepare pork chop
  12:23:40 INFO  [process_builder_dynamics::demo::choice] [Diner] Received pork chop. Spending 5 seconds eating it
  12:23:40 INFO  [process_builder_dynamics::demo::choice] [MainCourse] Served pork chop. Terminating main course process
  12:23:45 INFO  [process_builder_dynamics::demo::choice] [Diner] Finished eating pork chop
  12:23:45 INFO  [process_builder_dynamics::demo::choice] [Diner] Main course process terminated
  12:23:45 INFO  [process_builder_dynamics::demo::choice] [Diner] Spending 4 seconds in washroom
  12:23:49 INFO  [process_builder_dynamics::demo::choice] [Diner] Leaving restaurant
  12:23:49 INFO  [process_builder_dynamics] [Main] Main program terminating

 */
