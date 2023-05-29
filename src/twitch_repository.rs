use std::collections::HashMap;

use twitch_irc::{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    transport::tcp::{TCPTransport, TLS},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

use crate::{
    commands::{elo_check::elo_check, ping::ping, rank::rank},
    errors::{join_channels_error::JoinChannelsError, parse_command_error::ParseCommandError},
};

pub async fn join_channels(
    login_name: String,
    oauth_token: String,
    config: HashMap<String, String>,
) -> Result<(), JoinChannelsError> {
    let login_config =
        ClientConfig::new_simple(StaticLoginCredentials::new(login_name, Some(oauth_token)));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(login_config);

    for channel in config.keys() {
        client.join(channel.to_owned())?;
    }
    while let Some(message) = incoming_messages.recv().await {
        if let ServerMessage::Privmsg(message) = message {
            parse_command(message, &client, &config).await?;
        }
    }
    Ok(())
}

async fn parse_command(
    message: PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    config: &HashMap<String, String>,
) -> Result<(), ParseCommandError> {
    let command = match message.message_text.trim().split_once(' ') {
        Some((command, _)) => command,
        None => message.message_text.trim(),
    };

    if command == "!ping" {
        ping(client, message).await?;
    } else if command == "!rank" {
        rank(client, message, config).await?;
    } else if command == "!elocheck" {
        elo_check(client, message).await?;
    }
    Ok(())
}
