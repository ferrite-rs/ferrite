use crate::base::{
  Processes,
  Appendable,
  Reversible,
  EmptyList,
  ProcessNode,
  Inactive,
};

// Type level list manipulation tricks copied from https://github.com/lloydmeta/frunk

/*
  Concrete implementation for process linked list.
 */

impl Processes for () {
  type Values = ();
}

impl EmptyList for () {
  fn make_empty_list () -> () {
    return ();
  }
}

impl
  < R >
  EmptyList for
  ( Inactive, R )
where
  R : Processes + EmptyList
{
  fn make_empty_list () ->
    ((), < R as Processes > :: Values)
  {
    return (
      (),
      < R as EmptyList > :: make_empty_list()
    );
  }
}

impl
  < P, R >
  Processes for
  ( P, R )
where
  P: ProcessNode,
  R: Processes
{
  type Values =
    ( P :: NodeValue,
      R::Values
    );
}

impl <R: Processes> Appendable <R> for () {
  type AppendResult = R;

  fn append_channels(
    _: (),
    r: <R as Processes>::Values
  ) ->
    <R as Processes>::Values
  {
    return r;
  }

  fn split_channels(
    r: <R as Processes>::Values
  ) -> (
    (),
    <R as Processes>::Values
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
  Appendable < S > for
  ( P, R )
where
  P: ProcessNode,
  R: Processes,
  S: Processes,
  R: Appendable < S >
{
  type AppendResult =
    (
      P,
      < R as Appendable < S > > :: AppendResult
    );

  fn append_channels(
    (p, r) : (
      < P as ProcessNode > :: NodeValue,
      <R as Processes>::Values
    ),
    s : <S as Processes>::Values
  ) -> (
    < P as ProcessNode > :: NodeValue,
    <
      < R as Appendable<S> >::AppendResult
      as Processes
    >::Values
  ) {
    return (p, < R as Appendable<S> >::append_channels(r, s));
  }

  fn split_channels(
    (p, r): (
      < P as ProcessNode > :: NodeValue,
      <
        < R as Appendable<S> >::AppendResult
        as Processes
      >::Values
    )
  ) -> (
    < ( P, R ) as Processes > :: Values,
    <S as Processes>::Values
  ) {
    let (r2, s) = < R as Appendable<S> >::split_channels(r);
    return ((p, r2), s);
  }
}
