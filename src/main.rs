pub mod env;
mod errors;

mod commands;
mod model;
mod repository;
mod twitch_repository;

use env::Env;

use errors::run_error::RunError;
use repository::get_config;
use twitch_repository::join_channels;
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        println!("{}", error);
    }
}

async fn run() -> Result<(), RunError> {
    let env = Env::check_variables()?;
    let config = get_config()?;
    join_channels(env.bot_name, env.oauth_token, config).await?;
    Ok(())
}
