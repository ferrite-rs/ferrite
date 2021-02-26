use crate::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    AppendContext,
    Empty,
    EmptyContext,
    PartialSession,
    Protocol,
    Session,
    Slot,
  },
  functional::nat::{
    Nat,
    S,
    Z,
  },
};

pub fn new_session<A>(cont : Session<A>) -> Session<A>
where
  A : Protocol,
{
  cont
}

pub fn session<C, A>(cont : PartialSession<C, A>) -> Session<A>
where
  C : EmptyContext,
  A : Protocol,
{
  unsafe_create_session(move |(), sender| async move {
    let ctx = <C as EmptyContext>::empty_values();

    unsafe_run_session(cont, ctx, sender).await;
  })
}

pub fn partial_session<C, A>(cont : Session<A>) -> PartialSession<C, A>
where
  C : EmptyContext,
  A : Protocol,
{
  unsafe_create_session(move |_, sender| async move {
    unsafe_run_session(cont, (), sender).await
  })
}

pub fn append_emtpy_slot<C, A>(
  cont : PartialSession<C, A>
) -> PartialSession<C::Appended, A>
where
  A : Protocol,
  C : AppendContext<(Empty, ())>,
{
  unsafe_create_session(move |ctx1, sender| async move {
    let (ctx2, _) = C::split_context(ctx1);
    unsafe_run_session(cont, ctx2, sender).await
  })
}

pub fn session_1<A>(
  cont : impl FnOnce(Z) -> PartialSession<(Empty, ()), A>
) -> Session<A>
where
  A : Protocol,
{
  session(cont(Z::Value))
}

pub fn session_2<A>(
  cont : impl FnOnce(Z, S<Z>) -> PartialSession<(Empty, (Empty, ())), A>
) -> Session<A>
where
  A : Protocol,
{
  session(cont(Z::Value, <S<Z>>::Value))
}

pub fn partial_session_1<A, B>(
  cont : impl FnOnce(Z) -> PartialSession<(A, ()), B>
) -> PartialSession<(A, ()), B>
where
  A : Slot,
  B : Protocol,
{
  cont(Z::Value)
}

pub fn partial_session_2<A1, A2, B, F>(
  cont : impl FnOnce(Z, S<Z>) -> PartialSession<(A1, (A2, ())), B>
) -> PartialSession<(A1, (A2, ())), B>
where
  A1 : Slot,
  A2 : Slot,
  B : Protocol,
{
  cont(Z::Value, <S<Z>>::Value)
}
