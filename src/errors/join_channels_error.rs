use twitch_irc::validate;

use super::parse_command_error::ParseCommandError;

pub enum JoinChannelsError {
    JoinChannelError(validate::Error),
    ParseCommandError(ParseCommandError),
}

impl From<validate::Error> for JoinChannelsError {
    fn from(error: validate::Error) -> Self {
        JoinChannelsError::JoinChannelError(error)
    }
}
impl From<ParseCommandError> for JoinChannelsError {
    fn from(error: ParseCommandError) -> Self {
        JoinChannelsError::ParseCommandError(error)
    }
}

impl std::fmt::Display for JoinChannelsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JoinChannelsError::JoinChannelError(error) => write!(f, "{} ", error),
            JoinChannelsError::ParseCommandError(error) => write!(f, "{} ", error),
        }
    }
}
