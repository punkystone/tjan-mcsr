use twitch_irc::{
    login::StaticLoginCredentials,
    message::PrivmsgMessage,
    transport::tcp::{TCPTransport, TLS},
    TwitchIRCClient,
};

use crate::{errors::elo_check_error::EloCheckError, repository::get_rank};
pub async fn elo_check(
    client: &TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    message: PrivmsgMessage,
) -> Result<(), EloCheckError> {
    let arguments = message
        .message_text
        .trim()
        .split(' ')
        .collect::<Vec<&str>>();
    if let Some(name) = arguments.get(1) {
        client
            .say(
                message.channel_login,
                format!(
                    "@{} {} ",
                    message.sender.name,
                    get_rank(&name.to_string()).await?
                ),
            )
            .await?;
    }
    Ok(())
}
