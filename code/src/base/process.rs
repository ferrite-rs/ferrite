
/// A process / session type. This can be used as either input or output process.
pub trait Process
  : Send + 'static
{
  type Value : Sized + Send;
}

pub mod public {
  pub trait Process : super::Process {}
}