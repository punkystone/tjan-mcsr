use twitch_irc::{
    login::StaticLoginCredentials,
    transport::tcp::{TCPTransport, TLS},
    Error,
};
pub struct PingError {
    pub error: Error<TCPTransport<TLS>, StaticLoginCredentials>,
}

impl From<Error<TCPTransport<TLS>, StaticLoginCredentials>> for PingError {
    fn from(error: Error<TCPTransport<TLS>, StaticLoginCredentials>) -> Self {
        PingError { error }
    }
}
impl std::fmt::Display for PingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ", self.error)
    }
}
