
pub use crate::base::*;
pub use crate::process::nary_choice::*;

pub trait ExternalSum < I >
  : ProcessSum
where
  I : Processes
{
  type SessionSum;
}

pub trait InternalSessionSum
  < Lens, I, ParentSum, Out >
  : ProcessSum
where
  ParentSum : ProcessSum,
  Out : Process,
  I : Processes,
{
  type CurrentSession : 'static;

  type SessionSum : 'static;
}

pub trait InternalSessionCont
  < ParentSession, Lens, I, Parent, Out >
  : InternalSessionSum <
      Lens, I, Parent, Out
    >
where
  Parent : ProcessSum,
  Out : Process,
  I : Processes,
{
  type CurrentCont;

  type ContSum;

  fn make_cont_sum
    ( selector : Self :: SelectorSum,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
        >
    ) ->
      Self :: ContSum
  ;
}

impl
  < I, P >
  ExternalSum < I >
  for P
where
  P : Process,
  I : Processes
{
  type SessionSum =
    PartialSession < I, P >;
}

impl
  < I, P, R >
  ExternalSum < I >
  for Sum < P, R >
where
  P : Process,
  R : ExternalSum < I >,
  I : Processes,
{
  type SessionSum =
    Sum <
      PartialSession <
        I,
        P
      >,
      R :: SessionSum
    >;
}

impl
  < Lens, I, ParentSum, Out, P, Rest >
  InternalSessionSum <
    Lens, I, ParentSum, Out
  >
  for Sum < P, Rest >
where
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum,
  Rest :
    InternalSessionSum <
      Lens, I, ParentSum, Out
    > + 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P
    >,
{
  type CurrentSession =
    PartialSession <
      Lens :: Target,
      Out
    >;

  type SessionSum =
    Sum <
      PartialSession <
        Lens :: Target,
        Out
      >,
      Rest :: SessionSum
    >;
}

impl
  < ParentSession, Lens, I, ParentSum, Out, P, Rest >
  InternalSessionCont <
    ParentSession, Lens, I, ParentSum, Out
  >
  for Sum < P, Rest >
where
  P : Process + 'static,
  Out : Process + 'static,
  ParentSum : ProcessSum,
  ParentSession : 'static,
  Rest :
    InternalSessionCont <
      ParentSession, Lens, I, ParentSum, Out
    > + 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      InternalChoice < ParentSum >,
      P
    >,
{
  type CurrentCont =
    Box <
      dyn FnOnce (
        Self :: CurrentSession
      ) ->
        ParentSession
    >;

  type ContSum =
    Sum <
      Self :: CurrentCont,
      Rest :: ContSum
    >;


  fn make_cont_sum
    ( selector : Self :: SelectorSum,
      inject :
        Box <
          dyn FnOnce (
            Self :: SessionSum
          ) ->
            ParentSession
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
                Rest :: SessionSum
              ) ->
                ParentSession
            >
          = Box::new (
              move | session | {
                let session_sum
                  : Self :: SessionSum
                  = Sum::Inr ( session );

                inject ( session_sum )
              });

        let cont_sum
          : Rest :: ContSum
          = Rest :: make_cont_sum (
              selector2,
              inject2
            );

        Sum :: Inr ( cont_sum )
      }
    }
  }
}