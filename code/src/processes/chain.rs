use crate::base::{
  Empty,
  Context,
  EmptyContext,
  AppendContext,
  Reversible,
  Slot,
  Z,
  S,
};

// Type level list manipulation tricks copied from https://github.com/lloydmeta/frunk

/*
  Concrete implementation for process linked list.
 */

impl Context for () {
  type Values = ();

  type Length = Z;
}

impl EmptyContext for () {
  fn empty_values () {
    ()
  }
}

impl < R >
  EmptyContext for
  ( Empty, R )
where
  R : EmptyContext
{
  fn empty_values () ->
      ((), R::Values)
  {
    ( (), R:: empty_values() )
  }
}

impl
  < P, R >
  Context for
  ( P, R )
where
  P: Slot,
  R: Context
{
  type Values =
    ( P :: Value,
      R::Values
    );

  type Length = S < R::Length >;
}

impl <R: Context> AppendContext <R> for () {
  type Appended = R;

  fn append_context(
    _: (),
    r: <R as Context>::Values
  ) ->
    <R as Context>::Values
  {
    return r;
  }

  fn split_context (
    r: <R as Context>::Values
  ) -> (
    (),
    <R as Context>::Values
  ) {
    return ((), r)
  }
}

impl Reversible for () {
  type Reversed = ();

  fn reverse_channels(_: ()) {
    return ();
  }

  fn unreverse_channels(_: ()) {
    return ();
  }
}

impl
  < P, R, S >
  AppendContext < S > for
  ( P, R )
where
  P: Slot,
  R: Context,
  S: Context,
  R: AppendContext < S >
{
  type Appended =
    (
      P,
      < R as AppendContext < S > > :: Appended
    );

  fn append_context (
    (p, r) : (
      P :: Value,
      R ::Values
    ),
    s : <S as Context>::Values
  ) -> (
    < P as Slot > :: Value,
    <
      R :: Appended
      as Context
    >::Values
  ) {
    return (p, < R as AppendContext<S> >::append_context(r, s));
  }

  fn split_context (
    (p, r): (
      P :: Value,
      < R::Appended
        as Context
      > :: Values
    )
  ) -> (
    < ( P, R ) as Context > :: Values,
    < S as Context >::Values
  ) {
    let (r2, s) = R :: split_context(r);
    return ((p, r2), s);
  }
}
