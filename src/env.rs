use crate::errors::environment_variables_error::EnvironmentVariablesError;

#[derive(Clone)]
pub struct Env {
    pub bot_name: String,
    pub oauth_token: String,
}
impl Env {
    #[allow(clippy::missing_errors_doc)]
    pub fn check_variables() -> Result<Env, EnvironmentVariablesError> {
        Ok(Env {
            bot_name: std::env::var("BOT_NAME")?,
            oauth_token: std::env::var("OAUTH_TOKEN")?,
        })
    }
}
