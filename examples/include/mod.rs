use anyhow::Context;
use trovochat::{messages, UserConfig};

// some helpers for the demo
fn get_env_var(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("please set `{}`", key))
}

pub fn get_user_config() -> anyhow::Result<trovochat::UserConfig> {
    let name = get_env_var("TROVO_NAME")?;
    let token = get_env_var("TROVO_TOKEN")?;

    // you need a `UserConfig` to connect to Trovo
    let config = UserConfig::builder()
        // the name of the associated trovo account
        .name(name)
        // and the provided OAuth token
        .token(token)
        // and enable all of the advanced message signaling from Trovo
        .enable_all_capabilities()
        .build()?;

    Ok(config)
}

// channels can be either in the form of '#museun' or 'museun'. the crate will internally add the missing #
pub fn channels_to_join() -> anyhow::Result<Vec<String>> {
    let channels = get_env_var("TROVO_CHANNEL")?
        .split(',')
        .map(ToString::to_string)
        .collect();
    Ok(channels)
}

pub async fn handle_message(msg: messages::Commands<'_>) {
    use messages::Commands::*;
    // All sorts of messages
    match msg {
        // This is the one users send to channels
        Privmsg(msg) => println!("[{}] {}: {}", msg.channel(), msg.name(), msg.data()),

        // This one is special, if trovo adds any new message
        // types, this will catch it until future releases of
        // this crate add them.
        Raw(_) => {}

        // These happen when you initially connect
        IrcReady(_) => {}
        Ready(_) => {}
        Cap(_) => {}

        // and a bunch of other messages you may be interested in
        ClearChat(_) => {}
        ClearMsg(_) => {}
        GlobalUserState(_) => {}
        HostTarget(_) => {}
        Join(_) => {}
        Notice(_) => {}
        Part(_) => {}
        Ping(_) => {}
        Pong(_) => {}
        Reconnect(_) => {}
        RoomState(_) => {}
        UserNotice(_) => {}
        UserState(_) => {}
        Whisper(_) => {}

        _ => {}
    }
}
