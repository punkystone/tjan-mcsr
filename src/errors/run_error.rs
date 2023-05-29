use super::{
    environment_variables_error::EnvironmentVariablesError, get_config_error::GetConfigError,
    join_channels_error::JoinChannelsError,
};
#[allow(clippy::enum_variant_names)]
pub enum RunError {
    EnvironmentVariablesError(EnvironmentVariablesError),
    GetConfigError(GetConfigError),
    JoinChannelsError(JoinChannelsError),
}

impl From<EnvironmentVariablesError> for RunError {
    fn from(error: EnvironmentVariablesError) -> Self {
        RunError::EnvironmentVariablesError(error)
    }
}

impl From<GetConfigError> for RunError {
    fn from(error: GetConfigError) -> Self {
        RunError::GetConfigError(error)
    }
}

impl From<JoinChannelsError> for RunError {
    fn from(error: JoinChannelsError) -> Self {
        RunError::JoinChannelsError(error)
    }
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RunError::EnvironmentVariablesError(error) => write!(f, "{} ", error),
            RunError::GetConfigError(error) => write!(f, "{} ", error),
            RunError::JoinChannelsError(error) => write!(f, "{} ", error),
        }
    }
}
