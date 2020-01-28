use async_macros::join;

use crate::base::fix::*;
use crate::base::*;
use crate::process::fix2::*;
use async_std::task;
use async_std::sync::{ Sender, channel };

pub fn fix_session
  < F, I >
  ( cont:
      PartialSession <
        I,
        < F as
          TyCon <
            FixProcess < F >
          >
        > :: Type
      >
  ) ->
    PartialSession <
      I,
      FixProcess < F >
    >
where
  I : Processes + 'static,
  F : Process,
  F :
    TyCon <
      FixProcess < F >
    >,
  F :: Value :
    TyCon <
      Fix < F :: Value >,
      Type =
        < < F as
            TyCon <
              FixProcess < F >
            >
          > :: Type
          as Process
        > :: Value,
    >,
  < F as
    TyCon <
      FixProcess < F >
    >
  > :: Type : Process,
  < F :: Value as
    TyCon <
      Fix < F :: Value >
    >
  > :: Type :
    Send
{
  create_partial_session (
    async move |
      ins,
      sender1 :
        Sender <
          Fix < F :: Value >
        >
    | {
      let (sender2, receiver)
        : ( Sender <
              < < F as
                  TyCon <
                    FixProcess < F >
                  >
                > :: Type
                as Process
              > :: Value
            >
          , _
          )
        = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( fix ( val ) ).await;
      });

      let child2 = task::spawn(
        run_partial_session
          ( cont, ins, sender2
          ) );

      join!(child1, child2).await;
    })
}