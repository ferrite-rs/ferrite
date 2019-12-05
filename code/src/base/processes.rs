
/// A list of processes for input. It has multiple implementations including
/// [crate::base::Processes].
pub trait Processes {
  type Values : Sized + Send;
}

/// An ordered linked list of processes.
pub trait EmptyList : Processes {
  fn make_empty_list () ->
    < Self as Processes > :: Values;
}

pub trait
  Appendable < R > : Processes
where
  R : Processes
{
  type AppendResult : Processes;

  fn append_channels(
    channels1: <Self as Processes>::Values,
    channels2: <R as Processes>::Values
  ) ->
    <Self::AppendResult as Processes>::Values;

  fn split_channels(
    channels: <Self::AppendResult as Processes>::Values
  ) -> (
    <Self as Processes>::Values,
    <R as Processes>::Values
  );
}

pub trait Reversible : Processes {
  type Reversed : Processes;

  fn reverse_channels(
    channels: <Self as Processes>::Values,
  ) ->
    <Self::Reversed as Processes>::Values;

  fn unreverse_channels(
    channels: <Self::Reversed as Processes>::Values,
  ) ->
    Self::Values;
}
