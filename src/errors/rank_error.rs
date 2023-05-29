pub use twitch_irc::{
    login::StaticLoginCredentials,
    transport::tcp::{TCPTransport, TLS},
    Error,
};

use super::get_rank_error::GetRankError;

pub enum RankError {
    GetRankError(GetRankError),
    SendError(Error<TCPTransport<TLS>, StaticLoginCredentials>),
}

impl From<GetRankError> for RankError {
    fn from(error: GetRankError) -> Self {
        RankError::GetRankError(error)
    }
}

impl From<Error<TCPTransport<TLS>, StaticLoginCredentials>> for RankError {
    fn from(error: Error<TCPTransport<TLS>, StaticLoginCredentials>) -> Self {
        RankError::SendError(error)
    }
}

impl std::fmt::Display for RankError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RankError::GetRankError(error) => write!(f, "{} ", error),
            RankError::SendError(error) => write!(f, "{} ", error),
        }
    }
}
