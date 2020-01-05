use async_macros::join;
use std::mem::transmute;
use async_std::task;
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{
  FixProcess,
  HoleProcess,
  ProcessAlgebra,
};

use crate::base::{
  Process,
  Processes,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

pub fn fill_hole
  < Ins, F >
  ( fix_session :
      PartialSession < Ins, FixProcess < F > >
  )
  -> PartialSession < Ins, HoleProcess < F > >
where
  F : ProcessAlgebra < HoleProcess < F > > + 'static,
  Ins : Processes + 'static
{
  create_partial_session (
    async move |
      ins,
      sender1: Sender < Box < () > >
    | {
      let sender2 :
          Sender <
            Box <
              <
                <
                  F as
                  ProcessAlgebra < HoleProcess < F > >
                > :: ToProcess
                as Process
              > :: Value
            >
          >
        = unsafe {
          transmute(sender1)
        };

      run_partial_session
        ( fix_session, ins, sender2
        ).await;
    })
}

fn unsafe_transmute_receiver < S, T >
  (r : Receiver < S >)
  -> Receiver < T >
{
  unsafe {
    return transmute(r);
  }
}

pub fn read_hole
  < S, T, D, P, F, Lens >
  ( _ : Lens
  , fix_session : PartialSession < T, P >
  )
  ->
    PartialSession < S, P >
where
  F : ProcessAlgebra < HoleProcess < F > > + 'static,
  P : Process + 'static,
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  Lens : ProcessLens <
    S, T, D,
    HoleProcess < F >,
    FixProcess < F >
  >
{
  create_partial_session (
    async move |
      ins1,
      sender: Sender < P :: Value >
    | {
      let ( receiver1, ins2 )
        : ( Receiver < Box < () > >, _)
        = < Lens as
            ProcessLens <
              S, T, D,
              HoleProcess < F >,
              FixProcess < F >
            >
          > :: split_channels ( ins1 );

      let receiver2 :
        Receiver <
          Box <
            <
              <
                F as ProcessAlgebra < HoleProcess < F > >
              > :: ToProcess
              as Process
            > :: Value
          >
        >
        = unsafe_transmute_receiver(receiver1);

      let ins3 =
        < Lens as
          ProcessLens <
            S, T, D,
            HoleProcess < F >,
            FixProcess < F >
          >
        > :: merge_channels ( receiver2, ins2 );

        run_partial_session
          ( fix_session, ins3, sender
          ).await;
    })
}

pub fn fix_session
  < Ins, F >
  ( session: PartialSession <
      Ins,
      <
        F as ProcessAlgebra < HoleProcess < F > >
      > :: ToProcess
    >
  ) ->
    PartialSession <
      Ins,
      FixProcess < F >
    >
where
  F : ProcessAlgebra < HoleProcess < F > >
    + 'static,
  Ins : Processes + 'static
{
  create_partial_session (
    async move |
      ins,
      sender1:
        Sender <
          Box <
            <
              <
                F as ProcessAlgebra < HoleProcess < F > >
              > :: ToProcess
              as Process
            > :: Value
          >
        >
    | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send( Box::new(val) ).await;
      });

      let child2 = task::spawn(async move {
        run_partial_session
          ( session, ins, sender2
          ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn unfix_session
  < S, T, D, P, F, Lens >
  ( _ : Lens,
    session : PartialSession < T, P >
  )
  ->
    PartialSession < S, P >
where
  F : ProcessAlgebra < HoleProcess < F > > + 'static,
  P : Process + 'static,
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  Lens :
    ProcessLens <
      S, T, D,
      FixProcess < F >,
      < F as ProcessAlgebra < HoleProcess < F > >
      > :: ToProcess
    >
{
  create_partial_session (
    async move |
      ins1,
      sender1: Sender < P :: Value >
    | {
      let (receiver1, ins2)
        : ( Receiver <
              Box <
                <
                  <
                    F as ProcessAlgebra < HoleProcess < F > >
                  > :: ToProcess
                  as Process
                > :: Value
              >
            >,
            _
          )
        = < Lens as
            ProcessLens <
              S, T, D,
              FixProcess < F >,
              < F as ProcessAlgebra < HoleProcess < F > >
              > :: ToProcess
            >
          > :: split_channels ( ins1 );

      let (sender2, receiver2) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver1.recv().await.unwrap();
        sender2.send( *val ).await;
      });

      let ins3 =
        < Lens as
          ProcessLens <
            S, T, D,
            FixProcess < F >,
            < F as ProcessAlgebra < HoleProcess < F > >
            > :: ToProcess
          >
        > :: merge_channels ( receiver2, ins2 );

      let child2 = task::spawn(async move {
        run_partial_session
          ( session, ins3, sender1
          ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn unfix_hole
  < S, T1, D1, T2, D2, P, F, Lens >
  ( lens : Lens
  , session : PartialSession < T2, P >
  )
  ->
    PartialSession < S, P >
where
  F : ProcessAlgebra < HoleProcess < F > > + 'static,
  P : Process + 'static,
  S : Processes + 'static,
  T1 : Processes + 'static,
  D1 : Processes + 'static,
  T2 : Processes + 'static,
  D2 : Processes + 'static,
  Lens : Copy,
  Lens : ProcessLens <
    S, T1, D1,
    HoleProcess < F >,
    FixProcess < F >
  >,
  Lens :
    ProcessLens <
      T1, T2, D2,
      FixProcess < F >,
      < F as ProcessAlgebra < HoleProcess < F > >
      > :: ToProcess
    >
{
  read_hole ( lens,
    unfix_session ( lens,
      session ) )
}