pub trait SharedProtocol : Send + 'static
{ }

pub mod public {
  pub trait SharedProtocol : super::SharedProtocol {}
}

impl < A >
  public::SharedProtocol
  for A
where
  A : SharedProtocol
{}
