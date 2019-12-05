use std::pin::Pin;
use std::future::{ Future };

pub fn wrap_async < T >
  ( v : T ) ->
    Pin < Box < dyn Future <
      Output = T
    > + Send > >
where
  T : Send + 'static
{
  Box::pin ( async move {
    v
  })
}