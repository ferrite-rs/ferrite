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
  type Appended =
    (
      P,
      < R as AppendContext < S > > :: Appended
    );

  fn append_channels(
    (p, r) : (
      < P as Slot > :: Value,
      <R as Context>::Values
    ),
    s : <S as Context>::Values
  ) -> (
    < P as Slot > :: Value,
    <
      < R as AppendContext<S> >::Appended
      as Context
    >::Values
  ) {
    return (p, < R as AppendContext<S> >::append_channels(r, s));
  }

  fn split_channels(
    (p, r): (
      < P as Slot > :: Value,
      <
        < R as AppendContext<S> >::Appended
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
