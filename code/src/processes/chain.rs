use crate::base::{
  Empty,
  Context,
  EmptyContext,
  AppendContext,
  Reversible,
  Slot,
};

// Type level list manipulation tricks copied from https://github.com/lloydmeta/frunk

/*
  Concrete implementation for process linked list.
 */

impl Context for () {
  type Values = ();
}

impl EmptyContext for () {
  fn make_empty_list () -> () {
    return ();
  }
}

impl
  < R >
  EmptyContext for
  ( Empty, R )
where
  R : Context + EmptyContext
{
  fn make_empty_list () ->
    ((), < R as Context > :: Values)
  {
    return (
      (),
      < R as EmptyContext > :: make_empty_list()
    );
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
    ( P :: SlotValue,
      R::Values
    );
}

impl <R: Context> AppendContext <R> for () {
  type AppendResult = R;

  fn append_channels(
    _: (),
    r: <R as Context>::Values
  ) ->
    <R as Context>::Values
  {
    return r;
  }

  fn split_channels(
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
  type AppendResult =
    (
      P,
      < R as AppendContext < S > > :: AppendResult
    );

  fn append_channels(
    (p, r) : (
      < P as Slot > :: SlotValue,
      <R as Context>::Values
    ),
    s : <S as Context>::Values
  ) -> (
    < P as Slot > :: SlotValue,
    <
      < R as AppendContext<S> >::AppendResult
      as Context
    >::Values
  ) {
    return (p, < R as AppendContext<S> >::append_channels(r, s));
  }

  fn split_channels(
    (p, r): (
      < P as Slot > :: SlotValue,
      <
        < R as AppendContext<S> >::AppendResult
        as Context
      >::Values
    )
  ) -> (
    < ( P, R ) as Context > :: Values,
    <S as Context>::Values
  ) {
    let (r2, s) = < R as AppendContext<S> >::split_channels(r);
    return ((p, r2), s);
  }
}
