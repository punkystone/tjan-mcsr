use super::{elo_check_error::EloCheckError, ping_error::PingError, rank_error::RankError};
#[allow(clippy::enum_variant_names)]
pub enum ParseCommandError {
    PingError(PingError),
    RankError(RankError),
    EloCheckError(EloCheckError),
}

impl From<PingError> for ParseCommandError {
    fn from(error: PingError) -> Self {
        ParseCommandError::PingError(error)
    }
}

impl From<RankError> for ParseCommandError {
    fn from(error: RankError) -> Self {
        ParseCommandError::RankError(error)
    }
}

impl From<EloCheckError> for ParseCommandError {
    fn from(error: EloCheckError) -> Self {
        ParseCommandError::EloCheckError(error)
    }
}

impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseCommandError::PingError(error) => write!(f, "{} ", error),
            ParseCommandError::RankError(error) => write!(f, "{} ", error),
            ParseCommandError::EloCheckError(error) => write!(f, "{} ", error),
        }
    }
}
