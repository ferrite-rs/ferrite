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

// Applied level list manipulation tricks copied from https://github.com/lloydmeta/frunk

/*
  Concrete implementation for protocol linked list.
 */

impl Context for () {
  type Endpoints = ();

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
      ((), R::Endpoints)
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
  type Endpoints =
    ( P :: Endpoint,
      R::Endpoints
    );

  type Length = S < R::Length >;
}

impl <R: Context> AppendContext <R> for () {
  type Appended = R;

  fn append_context(
    _: (),
    r: <R as Context>::Endpoints
  ) ->
    <R as Context>::Endpoints
  {
    return r;
  }

  fn split_context (
    r: <R as Context>::Endpoints
  ) -> (
    (),
    <R as Context>::Endpoints
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
      P :: Endpoint,
      R ::Endpoints
    ),
    s : <S as Context>::Endpoints
  ) -> (
    < P as Slot > :: Endpoint,
    <
      R :: Appended
      as Context
    >::Endpoints
  ) {
    return (p, < R as AppendContext<S> >::append_context(r, s));
  }

  fn split_context (
    (p, r): (
      P :: Endpoint,
      < R::Appended
        as Context
      > :: Endpoints
    )
  ) -> (
    < ( P, R ) as Context > :: Endpoints,
    < S as Context >::Endpoints
  ) {
    let (r2, s) = R :: split_context(r);
    return ((p, r2), s);
  }
}
