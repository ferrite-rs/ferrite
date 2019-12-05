use async_macros::join;
use std::mem::transmute;
use async_std::task;
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{ FixProcess, HoleProcess };
use crate::base::*;

use crate::fix::*;

pub fn fill_hole
  < Ins, F >
  ( fix_session : PartialSession < Ins, FixProcess < F > >
  )
  -> PartialSession < Ins, HoleProcess < F > >
where
  F : AlgebraT < HoleProcess < F > > + 'static,
  <
    F as AlgebraT < HoleProcess < F > >
  > :: Algebra : Process,
  Ins : Processes + 'static
{
  PartialSession {
    builder: Box ::new (move |
      ins,
      sender1: Sender < Box < () > >
    | {
      let sender2 :
          Sender <
            Box <
              <
                <
                  F as AlgebraT < HoleProcess < F > >
                > :: Algebra
                as Process
              > :: Value
            >
          >
        = unsafe {
          transmute(sender1)
        };

      Box::pin(async {
        (fix_session.builder)(ins, sender2).await;
      })
    })
  }
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
  F : AlgebraT < HoleProcess < F > > + 'static,
  <
    F as AlgebraT < HoleProcess < F > >
  > :: Algebra : Process,
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
  PartialSession {
    builder: Box ::new ( move |
      ins1,
      sender: Sender < P :: Value >
    | {
      Box::pin(async {
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
                  F as AlgebraT < HoleProcess < F > >
                > :: Algebra
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

        (fix_session.builder)(ins3, sender).await;
      })
    })
  }
}

pub fn fix_session
  < Ins, F >
  ( session: PartialSession <
      Ins,
      <
        F as AlgebraT < HoleProcess < F > >
      > :: Algebra
    >
  ) ->
    PartialSession <
      Ins,
      FixProcess < F >
    >
where
  F : AlgebraT < HoleProcess < F > >
    + 'static,
  <
    F as AlgebraT < HoleProcess < F > >
  > :: Algebra : Process,
  Ins : Processes + 'static
{
  PartialSession {
    builder: Box ::new (move |
      ins,
      sender1:
        Sender <
          Box <
            <
              <
                F as AlgebraT < HoleProcess < F > >
              > :: Algebra
              as Process
            > :: Value
          >
        >
    | {
      Box::pin(async {
        let (sender2, receiver) = channel(1);

        let child1 = task::spawn(async move {
          let val = receiver.recv().await.unwrap();
          sender1.send( Box::new(val) ).await;
        });

        let child2 = task::spawn(async move {
          (session.builder)(ins, sender2).await;
        });

        join!(child1, child2).await;
      })
    })
  }
}

pub fn unfix_session
  < S, T, D, P, F, Lens >
  ( _ : Lens,
    session : PartialSession < T, P >
  )
  ->
    PartialSession < S, P >
where
  F : AlgebraT < HoleProcess < F > > + 'static,
  <
    F as AlgebraT < HoleProcess < F > >
  > :: Algebra : Process,
  P : Process + 'static,
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  Lens :
    ProcessLens <
      S, T, D,
      FixProcess < F >,
      < F as AlgebraT < HoleProcess < F > >
      > :: Algebra
    >
{
  PartialSession {
    builder: Box ::new (move |
      ins1,
      sender1: Sender < P :: Value >
    | {
      let (receiver1, ins2)
        : ( Receiver <
              Box <
                <
                  <
                    F as AlgebraT < HoleProcess < F > >
                  > :: Algebra
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
              < F as AlgebraT < HoleProcess < F > >
              > :: Algebra
            >
          > :: split_channels ( ins1 );
      Box::pin(async {
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
              < F as AlgebraT < HoleProcess < F > >
              > :: Algebra
            >
          > :: merge_channels ( receiver2, ins2 );

        let child2 = task::spawn(async move {
          (session.builder)(ins3, sender1).await;
        });

        join!(child1, child2).await;
      })
    })
  }
}
