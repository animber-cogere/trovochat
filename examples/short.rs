/* in your Cargo.toml
[dependencies]
trovochat = "0.8"                               # this crate
tokio = { version = "0.2", features = ["full"] } # you need tokio to run it
*/

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

    // run the client
    let done = client.run(read, write);

    // 'block' until we're connected
    let ready = client.wait_for_irc_ready().await.unwrap();
    eprintln!("your irc name: {}", ready.nickname);

    // the writer is also clonable
    client.writer().join("#museun").await.unwrap();

    // this resolves when the client disconnects
    // or is forced to stop with Client::stop
    use trovochat::client::Status;
    match done.await {
        // client was disconnected by the server
        Ok(Status::Eof) => {}
        // client was canceled by the user (`stop`)
        Ok(Status::Canceled) => {}
        // an error was received when trying to read or write
        Err(err) => eprintln!("error!: {}", err),
    };
}
