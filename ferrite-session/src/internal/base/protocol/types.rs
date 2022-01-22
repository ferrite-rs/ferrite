use crate::internal::functional::App;

pub struct ProviderEndpointF;

pub struct ClientEndpointF;

pub type ProviderEndpoint<A> = App<'static, ProviderEndpointF, A>;

pub type ClientEndpoint<A> = App<'static, ClientEndpointF, A>;
