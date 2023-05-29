use twitch_irc::{
    login::StaticLoginCredentials,
    transport::tcp::{TCPTransport, TLS},
    Error,
};

use super::get_rank_error::GetRankError;

pub enum EloCheckError {
    GetRankError(GetRankError),
    SendError(Error<TCPTransport<TLS>, StaticLoginCredentials>),
}

impl From<GetRankError> for EloCheckError {
    fn from(error: GetRankError) -> Self {
        EloCheckError::GetRankError(error)
    }
}

impl From<Error<TCPTransport<TLS>, StaticLoginCredentials>> for EloCheckError {
    fn from(error: Error<TCPTransport<TLS>, StaticLoginCredentials>) -> Self {
        EloCheckError::SendError(error)
    }
}

impl std::fmt::Display for EloCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EloCheckError::GetRankError(error) => write!(f, "{} ", error),
            EloCheckError::SendError(error) => write!(f, "{} ", error),
        }
    }
}
