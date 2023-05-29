use twitch_irc::{
    login::StaticLoginCredentials,
    message::PrivmsgMessage,
    transport::tcp::{TCPTransport, TLS},
    TwitchIRCClient,
};

use crate::errors::ping_error::PingError;

pub async fn ping(
    client: &TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    message: PrivmsgMessage,
) -> Result<(), PingError> {
    client
        .say(
            message.channel_login,
            format!("@{} MrDestructoid beep beep ", message.sender.name),
        )
        .await?;
    Ok(())
}
