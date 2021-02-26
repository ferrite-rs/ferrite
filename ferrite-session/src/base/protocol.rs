use crate::functional::nat::*;

pub trait Protocol: Send + 'static
{
}

impl Protocol for Z {}

impl<N> Protocol for S<N> where N : Protocol {}
