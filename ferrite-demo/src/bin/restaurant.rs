use std::time::Duration;

use ferrite_session::*;
use tokio::time::sleep;

pub struct MushroomSoup {}

pub struct TomatoSoup {}

pub struct BeefSteak {}

pub struct PorkChop {}

define_choice! {
  SoupMenu ;
  MushroomMenu: SendValue < MushroomSoup, End >,
  TomatoMenu: SendValue < TomatoSoup, End >,
}

define_choice! {
  MainMenu ;
  BeefMenu: SendValue < BeefSteak, End >,
  PorkMenu: SendValue < PorkChop, End >,
}

pub fn restaurant_session() -> Session<End>
{

  let soup_of_the_day : Session<InternalChoice<SoupMenu>> = offer_case(
    MushroomMenuLabel,
    send_value!(
      {

        println!("[Soup] Spending 3 seconds to prepare mushroom soup");

        sleep(Duration::from_secs(2)).await;

        println!("[Soup] Finished preparing mushroom soup");

        MushroomSoup {}
      },
      terminate!({

        println!("[Soup] Served mushroom soup. Terminating soup protocol");
      })
    ),
  );

  let main_dish : Session<ExternalChoice<MainMenu>> = offer_choice! {
    BeefMenu => {
      println!("[MainCourse] Customer chose to eat beef steak");

      send_value!(
        {
          println!("[MainCourse] Spending 7 seconds to prepare beef steak");
          sleep(Duration::from_secs(7)).await;

          BeefSteak{}

        },
        terminate! ({
          println!("[MainCourse] Served beef steak. Terminating main course protocol");
        }))
    }
    PorkMenu => {
      println!("[MainCourse] Customer chose to eat pork chop");

      send_value! (
        {
          println!("[MainCourse] Spending 5 seconds to prepare pork chop");
          sleep(Duration::from_secs(5)).await;

          PorkChop{}
        },
        terminate! ({
          println!("[MainCourse] Served pork chop. Terminating main course protocol");
        }) )
    }
  };

  let menu : Session<
    SendChannel<InternalChoice<SoupMenu>, ExternalChoice<MainMenu>>,
  > = include_session! ( soup_of_the_day, chan => {
    send_channel_from ( chan,
      partial_session( main_dish ) )
  });

  let diner : Session<
    ReceiveChannel<
      SendChannel<InternalChoice<SoupMenu>, ExternalChoice<MainMenu>>,
      End,
    >,
  > = receive_channel! ( menu_chan => {
      receive_channel_from! ( menu_chan, soup_chan => {
        case! { soup_chan ;
          MushroomMenu => {
            println!("[Diner] Restaurant offers mushroom soup today");

            receive_value_from! ( soup_chan, _mushroom_soup => {
              println!("[Diner] Received mushroom soup. Spending 2 seconds drinking it");
              sleep(Duration::from_secs(2)).await;
              println!("[Diner] Finished drinking mushroom soup");

              println!("[Diner] Choosing pork chop for main");

              wait! ( soup_chan, {
                println!("[Diner] Soup protocol terminated");

                choose! ( menu_chan, PorkMenu,
                  receive_value_from! ( menu_chan, _pork_chop => {
                    println!("[Diner] Received pork chop. Spending 5 seconds eating it");
                    sleep(Duration::from_secs(5)).await;
                    println!("[Diner] Finished eating pork chop");

                    wait! ( menu_chan, {
                      println!("[Diner] Main course protocol terminated");

                      terminate! ({
                        println!("[Diner] Spending 4 seconds in washroom");
                        sleep(Duration::from_secs(4)).await;
                        println!("[Diner] Leaving restaurant");
                      })
                    })
                  })
                )
              })
            })
          }
          TomatoMenu => {
            println!("[Diner] Restaurant offers tomato soup today");

            receive_value_from! ( soup_chan, _tomato_soup => {
              println!("[Diner] Received tomato soup. Spending 1 second drinking it");

              sleep(Duration::from_secs(1)).await;

              println!("[Diner] finished drinking tomato soup");
              println!("[Diner] Choosing beef steak for main");

              wait! ( soup_chan, {
                println!("[Diner] Soup protocol terminated");

                choose! ( menu_chan, BeefMenu,
                  receive_value_from! ( menu_chan, _beef_steak => {
                    println!("[Diner] Received beef steak. Spending 6 seconds eating it");
                    sleep(Duration::from_secs(6)).await;
                    println!("[Diner] Finished eating beef steak.");

                    wait! ( menu_chan, {
                      println!("[Diner] Main course protocol terminated");

                      terminate! ({
                        println!("[Diner] Spending 3 seconds in washroom");
                        sleep(Duration::from_secs(3)).await;
                        println!("[Diner] Leaving restaurant");
                      })
                    })
                  }))
              })
            })
          }
      }
    })
  });

  let restaurant = apply_channel(diner, menu);

  return restaurant;
}

#[tokio::main]

pub async fn main()
{

  run_session(restaurant_session()).await;
}
