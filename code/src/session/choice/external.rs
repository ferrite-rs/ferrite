use async_std::task;
use async_std::sync::{ Receiver, channel };

use crate::base::{
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::process::{
  Either,
  Choice,
  ExternalChoice
};

pub struct ExternalChoiceResult < L, R >
{
  result: Either< L, R >
}

fn left_choice < L, R > (res: L)
  -> ExternalChoiceResult< L, R >
{
  return ExternalChoiceResult {
    result: Either::Left(res)
  }
}

fn right_choice < L, R > (res: R)
  -> ExternalChoiceResult< L, R >
{
  return ExternalChoiceResult {
    result: Either::Right(res)
  }
}

/*
  Additive Conjunction / External Choice

    cont_builder(Left) :: Δ ⊢ A  cont_builder(Right) :: Δ  ⊢ B
  ==============================================================
          offer_choice(cont_builder) Δ ⊢  :: (x : A & B)

  Takes an offer builder that builds either the left process or the right process.

  With dependent type, the offer builder function is supposed to be something like:

    data Choice = Left | Right

    choiceApplied :: Applied -> Applied -> Choice -> Applied
    choiceApplied a _ Left = a
    choiceApplied _ b Right = b

    ContBuilder :: (a: Applied) -> (b: Applied) -> (c: Choice) -> (choiceApplied a b c)

  But we don't really have dependent type in Rust. Fortunately there is a
  way to emulate dependent type indexed by types with finite terms, i.e. Bool.

  Using continuation passing style, we can define offer builder as follow:

    ContBuilder :: forall a b r. Either (a -> r) (b -> r) -> r

  This way we encode the choice together with the continuation to produce a result.
  In order to produce a generic r, offer_builder have no choice but to produce
  either an a or a b depending on the value of the Either type.

  There is just one more small issue which is that we also can't really do
  impredicative polymorphism, e.g. generic closure, in Rust. To work around
  that we define an opaque r type ExternalChoiceResult with private constructors,
  so that user code can never construct a ExternalChoiceResult on their own.

    cont_builder :: forall a b
      . Either (a -> ExternalChoiceResult) (b -> ExternalChoiceResult)
      -> ExternalChoiceResult

  With that we can call offer_builder with our choice, and be confident that
  offer_builder will produce the a or b that we want and extract it from the
  ExternalChoiceResult.

  offerChoice
      :: forall ctx p q .
      ( Protocol p
      , Protocol q
      , Context ctx
      )
    => (forall r
        . Either
            (Receiver (Session ctx p) -> r)
            (Receiver (Session ctx q) -> r)
        -> Either (Session ctx p) (Session ctx q)
       )
    ->  PartialSession ctx (ExternalChoice p q)
 */
pub type ReturnChoice < C, P, Q > =
  Either <
    Box < dyn FnOnce (
       PartialSession < C, P >
    ) -> ExternalChoiceResult <
       PartialSession < C, P >,
       PartialSession < C, Q >
    > + Send >,
    Box< dyn FnOnce (
       PartialSession < C, Q >
    ) -> ExternalChoiceResult <
       PartialSession < C, P >,
       PartialSession < C, Q >
    > + Send >
  >;

pub fn offer_choice < C, P, Q, F >
  ( cont_builder : F )
  ->
    PartialSession <
      C,
      ExternalChoice < P, Q >
    >
where
  P   : Protocol,
  Q   : Protocol,
  C : Context,
  F   : FnOnce(
          ReturnChoice < C, P, Q >
        ) -> ExternalChoiceResult<
           PartialSession < C, P >,
           PartialSession < C, Q >
        > + Send + 'static
{
  unsafe_create_session (
    async move | ctx, sender | {
      sender.send ( ExternalChoice (
        Box::new(
          move |choice : Choice| ->
            Either<
              Receiver < P >,
              Receiver < Q >
            >
          {
            match choice {
              Choice::Left => {
                let in_choice :
                  ReturnChoice < C, P, Q >
                = Either::Left(
                  Box::new (
                    left_choice
                  )
                );

                let cont_variant = cont_builder(in_choice).result;

                match cont_variant {
                  Either::Left(cont) => {
                    let (sender, receiver) = channel(1);

                    task::spawn(async {
                      unsafe_run_session
                        ( cont, ctx, sender
                        ).await;
                    });

                    return Either::Left(receiver);
                  },

                  Either::Right(_) => {
                    panic!("expected cont_builder to provide left result");
                  }
                }
              },

              Choice::Right => {
                let in_choice : ReturnChoice <C, P, Q>
                = Either::Right(Box::new(right_choice));

                let cont_variant = cont_builder(in_choice).result;

                match cont_variant {
                  Either::Left(_) => {
                    panic!("expected cont_builder to provide right result");
                  },

                  Either::Right(cont) => {
                    let (sender, receiver) = channel(1);

                    task::spawn(async {
                      unsafe_run_session
                        ( cont, ctx, sender
                        ).await;
                    });

                    return Either::Right(receiver);
                  }
                }
              }
            }
          })
      ) ).await;
    })
}

/*
           cont ::  Δ, P, Δ'  ⊢ S
  =========================================
    choose_left(cont) :: Δ, P & Q, Δ' ⊢ S
 */

pub fn choose_left
  < N, I, P1, P2, S >
  ( _ : N,
    cont:
      PartialSession <
        N :: Target,
        S
      >
  ) ->
    PartialSession <
      I, S
    >
where
  I : Context,
  P1 : Protocol,
  P2 : Protocol,
  S : Protocol,
  N :
    ContextLens <
      I,
      ExternalChoice< P1, P2 >,
      P1
    >
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (offerer_chan, ctx2) = N :: extract_source ( ctx1 );

      let ExternalChoice ( offerer ) = offerer_chan.recv().await.unwrap();
      let input_variant = offerer ( Choice::Left );

      match input_variant {
        Either::Left(input_chan) => {
          let ctx3 =
            N :: insert_target( input_chan, ctx2 );

            unsafe_run_session
              ( cont, ctx3, sender
              ).await;
        },
        Either::Right(_) => {
          // this should never reach if offer_choice is implemented correctly
          panic!("expected offerer to provide right result");
        }
      }
    })
}

/*
           cont ::  Δ, Q, Δ'  ⊢ S
  =========================================
    choose_right(cont) :: Δ, P & Q, Δ' ⊢ S
 */
pub fn choose_right
  < N, I, P1, P2, S >
  ( _ : N,
    cont:
      PartialSession <
        N :: Target,
        S
      >
  ) ->
    PartialSession < I, S >
where
  I : Context,
  P1 : Protocol,
  P2 : Protocol,
  S : Protocol,
  N :
    ContextLens <
      I,
      ExternalChoice< P1, P2 >,
      P2
    >
{
  unsafe_create_session (
    async move | ctx1, sender | {
      let (offerer_chan, ctx2) = N :: extract_source ( ctx1 );

      let ExternalChoice ( offerer )
        = offerer_chan.recv().await.unwrap();

      let input_variant = offerer ( Choice::Right );

      match input_variant {
        Either::Left (_) => {
          // this should never reach if offer_choice is implemented correctly
          panic!("expected offerer to provide right result");
        },
        Either::Right (input_chan) => {
          let ctx3 =
            < N as
              ContextLens <
                I,
                ExternalChoice < P1, P2 >,
                P2
              >
            >
            :: insert_target( input_chan, ctx2 );

          unsafe_run_session
            ( cont, ctx3, sender
            ).await;
        }
      }
    })
}
