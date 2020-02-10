# trovochat
[![Documentation][docs_badge]][docs]
[![Crates][crates_badge]][crates]
[![Actions][actions_badge]][actions]

This crate provides a way to interace with [Trovo]'s chat.

Along with the messages as Rust types, it provides methods for sending messages.

## Demonstration
See [examples/demo.rs][demo] for a larger example

### A client
```rust
use trovochat::{events, Client, Secure};

// so .next() can be used on the EventStream
// futures::stream::StreamExt will also work
use tokio::stream::StreamExt as _;

#[tokio::main]
async fn main() {
    let (nick, pass) = trovochat::ANONYMOUS_LOGIN;
    let (read, write) = trovochat::connect_easy(&nick, &pass, Secure::UseTls)
        .await
        .unwrap();

    let client = Client::new();

    // client is clonable and can be sent across tasks
    let bot = client.clone();
    tokio::task::spawn(async move {
        // subscribe to 'PRIVMSG' events, this is a `Stream`
        let mut privmsgs = bot.dispatcher().await.subscribe::<events::Privmsg>();
        // 'msg' is a trovochat::messages::Privmsg<'static> here
        while let Some(msg) = privmsgs.next().await {
            eprintln!("[{}] {}: {}", msg.channel, msg.name, msg.data);
        }
    });

    // the writer is also clonable
    client.writer().join("#museun").await.unwrap();

    // this resolves when the client disconnects
    // or is forced to stop with Client::stop
    use trovochat::client::Status;
    match client.run(read, write).await {
        // client was disconnected by the server
        Ok(Status::Eof) => {}
        // client was canceled by the user (`stop`)
        Ok(Status::Canceled) => {}
        // an error was received when trying to read or write
        Err(err) => eprintln!("error!: {}", err),
    };
}
```


### Parsing messages
```rust
use trovochat::messages::*;
use trovochat::{AsOwned as _, Parse as _};

fn main() {
    let input = "@badge-info=subscriber/8;color=#59517B;tmi-sent-ts=1580932171144;user-type= :tmi.trovo.tv USERNOTICE #justinfan1234\r\n";

    // parse potentially many messages from the input (flatten just safely unwraps the result)
    // msg is a decode::Message<'a> here
    for msg in trovochat::decode(&input).flatten() {
        // parse message into a specific type
        let user_notice = UserNotice::parse(&msg).unwrap();
        // create an owned ('static) version of the message
        let owned: UserNotice<'static> = user_notice.as_owned();
        assert_eq!(user_notice, owned);

        // or parse the message into a 'All' type
        match AllCommands::parse(&msg).unwrap() {
            AllCommands::UserNotice(notice) => {
                // user_notice is a messages::UserNotice here
                assert_eq!(user_notice, notice);
            }
            _ => {}
        }

        // the tags are parsed and are accessible as methods
        // colors can be parsed into rgb/named types
        assert_eq!(
            user_notice.color().unwrap(),
            "#59517B".parse::<trovochat::color::Color>().unwrap()
        );

        // you can manually get tags from the message
        let ts = user_notice.tags.get("tmi-sent-ts").unwrap();
        assert_eq!(ts, "1580932171144");

        // or as a type
        let ts = user_notice
            .tags
            .get_parsed::<_, u64>("tmi-sent-ts")
            .unwrap();
        assert_eq!(ts, 1580932171144);
    }

    // parse one message at a time
    // this returns the index of the start of the possible next message
    let input =
        ":tmi.trovo.tv PING 1234567\r\n:museun!museun@museun.tmi.trovo.tv JOIN #museun\r\n";

    let (d, left) = trovochat::decode_one(input).unwrap();
    assert!(d > 0);
    assert_eq!(left.command, "PING");

    // use the new index
    let (i, right) = trovochat::decode_one(&input[d..]).unwrap();
    assert_eq!(i, 0);
    assert_eq!(right.command, "JOIN");
}
```

## License
`trovochat` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE][APACHE] and [LICENSE-MIT][MIT] for details.

[docs_badge]: https://docs.rs/trovochat/badge.svg
[docs]: https://docs.rs/trovochat
[crates_badge]: https://img.shields.io/crates/v/trovochat.svg
[crates]: https://crates.io/crates/trovochat
[actions_badge]: https://github.com/museun/trovochat/workflows/Rust/badge.svg
[actions]: https://github.com/museun/trovochat/actions

[demo]: ./examples/demo.rs

[APACHE]: ./LICENSE-APACHE
[MIT]: ./LICENSE-MIT
[Trovo]: https://dev.trovo.tv
