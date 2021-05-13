use crate::internal::functional::nat::*;

pub trait Protocol: Send + 'static
{
}

pub trait SharedProtocol: Send + 'static
{
}

impl Protocol for Z {}

impl<N> Protocol for S<N> where N: Protocol {}
