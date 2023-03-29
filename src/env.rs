use crate::errors::environment_variables_error::EnvironmentVariablesError;

#[derive(Clone)]
pub struct Env {
    pub port: u16,
}
impl Env {
    #[allow(clippy::missing_errors_doc)]
    pub fn check_variables() -> Result<Env, EnvironmentVariablesError> {
        Ok(Env {
            port: std::env::var("PORT")?.parse::<u16>()?,
        })
    }
}
