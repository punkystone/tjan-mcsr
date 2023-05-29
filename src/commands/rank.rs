use std::collections::HashMap;

use twitch_irc::{
    login::StaticLoginCredentials,
    message::PrivmsgMessage,
    transport::tcp::{TCPTransport, TLS},
    TwitchIRCClient,
};

use crate::{errors::rank_error::RankError, repository::get_rank};
pub async fn rank(
    client: &TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    message: PrivmsgMessage,
    config: &HashMap<String, String>,
) -> Result<(), RankError> {
    if let Some(name) = config.get(&message.channel_login) {
        client
            .say(
                message.channel_login,
                format!("@{} {} ", message.sender.name, get_rank(name).await?),
            )
            .await?;
    }
    Ok(())
}
