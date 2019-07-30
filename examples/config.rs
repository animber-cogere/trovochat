fn main() {
    use trovochat::commands;
    use trovochat::*;

    // use an anonymous login (you should probably use your name and your chat oauth token)
    let (nick, token) = trovochat::ANONYMOUS_LOGIN;
    let config = UserConfig::builder()
        .token(token) // your oauth token
        .nick(nick) // your nickname
        .commands() // command capabilites (see: https://dev.trovo.tv/docs/irc/commands/ )
        .membership() // command capabilites (see: https://dev.trovo.tv/docs/irc/membership/ )
        .tags() // command capabilites (see: https://dev.trovo.tv/docs/irc/tags/ )
        .build() // verify the settings
        .unwrap();

    // connect with the config
    let client = trovochat::connect(&config)
        .unwrap()
        .filter::<commands::PrivMsg>();
    let writer = client.writer();

    for event in client {
        match event {
            Event::IrcReady(..) => writer.join("museun").unwrap(),
            Event::Message(Message::PrivMsg(msg)) => {
                println!("{}: {}", msg.user(), msg.message());
            }
            Event::Error(..) => break,
            _ => continue,
        }
    }
}
