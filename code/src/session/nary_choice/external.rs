
use async_std::task;
pub use crate::base::*;
pub use crate::processes::*;
pub use crate::process::nary_choice::*;
use async_std::sync::{ Sender, Receiver, channel };

pub trait ExternalSum < I >
  : ProtocolSum2
where
  I : Context
{
  type CurrentSession : Send + 'static;

  type SessionSum : Send + 'static;
}

pub trait ExternalCont
  < ParentSession, I >
  : ExternalSum < I >
where
  I : Context
{
  type CurrentCont : Send + 'static;

  type ContSum : Send + 'static;

  fn make_cont_sum
    ( selector : Self :: SelectorSum,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
          + Send
        >
    ) ->
      Self :: ContSum
  ;

  fn run_session_sum
    ( ins : I :: Values,
      session_sum : Self :: SessionSum
    ) ->
      Self :: ValueSum
  ;
}

pub trait ExternalSelect
  < N, I, Source, Selector >
  : ExternalSum < I >
where
  I : Context,
  Source : Protocol,
  N :
    ContextLens <
      I,
      Source,
      Self :: SelectedProtocol
    >,
{
  type SelectedProtocol : Protocol;

  fn to_selector_sum ()
    -> Self :: SelectorSum
  ;

  fn select_ins (
    val_sum : Self :: ValueSum,
    ins :
      < N :: Deleted
        as Context
      > :: Values
  ) ->
    < N :: Target
      as Context
    > :: Values
  ;
}

pub struct ExternalChoiceResult
  < I, Sum >
where
  Sum : ExternalSum < I >,
  I : Context,
{
  result : Sum :: SessionSum
}

fn mk_external_choice_result
  < I, Sum >
  ( session_sum : Sum :: SessionSum
  ) ->
    ExternalChoiceResult <
      I, Sum
    >
where
  Sum : ExternalSum < I >,
  I : Context,
{
  ExternalChoiceResult {
    result : session_sum
  }
}

impl
  < I, P >
  ExternalSum < I >
  for P
where
  P : Protocol,
  I : Context
{
  type CurrentSession =
    PartialSession < I, P >;

  type SessionSum = Self :: CurrentSession;
}

impl
  < I, P, R >
  ExternalSum < I >
  for Sum < P, R >
where
  P : Protocol,
  I : Context,
  R : ExternalSum < I > + 'static,
{
  type CurrentSession =
    PartialSession <
      I,
      P
    >;

  type SessionSum =
    Sum <
      Self :: CurrentSession,
      R :: SessionSum
    >;
}

impl
  < ParentSession, I, P >
  ExternalCont <
    ParentSession, I
  > for P
where
  P : Protocol,
  I : Context,
  ParentSession : 'static,
{
  type CurrentCont =
    Box <
      dyn FnOnce (
        Self :: CurrentSession
      ) ->
        ParentSession
      + Send
    >;

  type ContSum =
    Self :: CurrentCont;


  fn make_cont_sum
    ( _ : Z,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
          + Send
        >
    ) ->
      Self :: ContSum
  {
    inject
  }

  fn run_session_sum
    ( ins : I :: Values,
      session : Self :: CurrentSession
    ) ->
      Self :: ValueSum
  {
    let (sender2, receiver2) = channel(1);

    task::spawn(async {
      run_partial_session (
        session,
        ins,
        sender2
      ).await;
    });

    receiver2
  }
}

impl
  < ParentSession, I, P, R >
  ExternalCont
    < ParentSession, I >
  for Sum < P, R >
where
  P : Protocol,
  I : Context,
  ParentSession : 'static,
  R :
    ExternalCont <
      ParentSession, I
    >
    + 'static,
{
  type CurrentCont =
    Box <
      dyn FnOnce (
        Self :: CurrentSession
      ) ->
        ParentSession
      + Send
    >;

  type ContSum =
    Sum <
      Self :: CurrentCont,
      R :: ContSum
    >;

  fn make_cont_sum
    ( selector : Self :: SelectorSum,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
          + Send
        >
    ) ->
      Self :: ContSum
  {
    match selector {
      Sum::Inl (_) => {
        let cont
          : Self :: CurrentCont
          = Box::new (
              move | session | {
                let session_sum
                  : Self :: SessionSum
                  = Sum::Inl ( session );

                let parent_session
                  : ParentSession
                  = inject ( session_sum );

                parent_session
              });

        let cont_sum
          : Self :: ContSum
          = Sum :: Inl ( cont );

        cont_sum
      },
      Sum::Inr (selector2) => {
        let inject2
          : Box <
              dyn FnOnce (
                R :: SessionSum
              ) ->
                ParentSession
              + Send
            >
          = Box::new (
              move | session | {
                let session_sum
                  : Self :: SessionSum
                  = Sum::Inr ( session );

                inject ( session_sum )
              });

        let cont_sum
          : R :: ContSum
          = R :: make_cont_sum (
              selector2,
              inject2
            );

        Sum :: Inr ( cont_sum )
      }
    }
  }

  fn run_session_sum
    ( ins : I :: Values,
      session_sum : Self :: SessionSum
    ) ->
      Self :: ValueSum
  {
    match session_sum {
      Sum::Inl (session) => {
        let (sender2, receiver2) = channel(1);

        task::spawn(async {
          run_partial_session (
            session,
            ins,
            sender2
          ).await;
        });

        Sum::Inl ( receiver2 )
      },
      Sum::Inr (session_sum2) => {
        Sum::Inr (
          R :: run_session_sum (
            ins, session_sum2
          ))
      }
    }
  }
}

impl
  < N, I, Source, P >
  ExternalSelect <
    N, I, Source, Z
  >
  for P
where
  P : Protocol,
  I : Context,
  Source : Protocol,
  N :
    ContextLens <
      I,
      Source,
      P
    >,
{
  type SelectedProtocol = P;

  fn to_selector_sum ()
    -> Self :: SelectorSum
  {
    Z {}
  }

  fn select_ins (
    receiver :
      Receiver < P :: Payload >,
    ins :
      < N :: Deleted
        as Context
      > :: Values
  ) ->
    < N :: Target
      as Context
    > :: Values
  {
    N :: merge_channels
      ( receiver, ins )
  }
}

impl
  < N, I, Source, Selector, P, Rest >
  ExternalSelect <
    N, I, Source,
    S < Selector >
  >
  for Sum < P, Rest >
where
  P : Protocol,
  I : Context,
  Source : Protocol,
  Rest : ExternalSelect <
    N, I, Source, Selector
  > + 'static,
  N :
    ContextLens <
      I,
      Source,
      Rest :: SelectedProtocol
    >,
{
  type SelectedProtocol =
    Rest :: SelectedProtocol;

  fn to_selector_sum ()
    -> Self :: SelectorSum
  {
    Sum::Inr (
      Rest :: to_selector_sum ()
    )
  }

  fn select_ins (
    val_sum : Self :: ValueSum,
    ins :
      < N :: Deleted
        as Context
      > :: Values
  ) ->
    < N :: Target
      as Context
    > :: Values
  {
    match val_sum {
      Sum::Inl (_) => {
        panic!(
          "impossible happened: received mismatch value_sum");
      },
      Sum::Inr (val_sum2) => {
        Rest :: select_ins
          ( val_sum2, ins )
      }
    }
  }
}

impl
  < N, I, Source, P, Rest >
  ExternalSelect <
    N, I, Source,
    Z
  >
  for Sum < P, Rest >
where
  P : Protocol,
  I : Context,
  Source : Protocol,
  Rest : ExternalSum < I > + 'static,
  N :
    ContextLens <
      I,
      Source,
      P
    >,
{
  type SelectedProtocol = P;

  fn to_selector_sum ()
    -> Self :: SelectorSum
  {
    Sum::Inl (
      Self :: select_current ()
    )
  }

  fn select_ins (
    val_sum : Self :: ValueSum,
    ins :
      < N :: Deleted
        as Context
      > :: Values
  ) ->
    < N :: Target
      as Context
    > :: Values
  {
    match val_sum {
      Sum::Inl (receiver) => {
        N :: merge_channels
          ( receiver, ins )
      },
      Sum::Inr (_) => {
        panic!(
          "impossible happened: received mismatch value_sum");
      }
    }
  }
}

pub fn offer_choice
  < I, Sum, F >
  ( cont : F
  ) ->
    PartialSession <
      I,
      ExternalChoice < Sum >
    >
where
  I : Context,
  Sum : ExternalSum < I > + 'static,
  Sum :
    ExternalCont <
      ExternalChoiceResult <
        I, Sum
      >,
      I
    >,
  F :
    FnOnce (
      Sum :: ContSum
    ) ->
      ExternalChoiceResult <
        I, Sum
      >
    + Send + 'static
{
  unsafe_create_session (
    async move |
      ins,
      sender :
        Sender <
          Box <
            dyn FnOnce
              ( Sum :: SelectorSum
              ) ->
                Sum :: ValueSum
            + Send
          >
        >
    | {
      let cont1 =
        move |
          selector : Sum :: SelectorSum
        | ->
          Sum :: ValueSum
        {
          let cont_sum
            : Sum :: ContSum
            = Sum :: make_cont_sum
                ( selector,
                  Box::new (
                    mk_external_choice_result)
                );

          let choice_res
            = cont (cont_sum);

          let val_sum
            = Sum :: run_session_sum
              ( ins,
                choice_res.result
              );

          val_sum
        };

      sender.send ( Box::new ( cont1 )).await;
    })
}

pub fn choose
  < N, Selector, I, Sum, P >
  ( _ : N,
    _ : Selector,
    cont :
      PartialSession <
        N :: Target,
        P
      >
  ) ->
    PartialSession <
      I, P
    >
where
  P : Protocol,
  I : Context,
  Sum :
    ExternalSelect <
      N,
      I,
      ExternalChoice < Sum >,
      Selector
    >,
  N :
    ContextLens <
      I,
      ExternalChoice < Sum >,
      Sum :: SelectedProtocol,
    >,
{
  unsafe_create_session (
    async move | ins1, sender | {
      let (receiver, ins2) =
        N :: split_channels ( ins1 );

      let offerer =
        receiver.recv().await.unwrap();

      let value_sum =
        offerer (
          Sum::to_selector_sum()
        );

      let ins3 =
        Sum :: select_ins (
          value_sum,
          ins2
        );

      run_partial_session
        ( cont, ins3, sender
        ).await;
    })
}
