/* in your Cargo.toml
[dependencies]
trovochat = "0.8"                               # this crate
tokio = { version = "0.2", features = ["full"] } # you need tokio to run it
*/

use tokio::stream::StreamExt as _; // for .next()

#[tokio::main]
async fn main() {
    let (nick, pass, channel) = (
        std::env::var("TROVO_NICK").unwrap(),
        std::env::var("TROVO_PASS").unwrap(),
        std::env::var("TROVO_CHANNEL").unwrap(),
    );

    let stream = trovochat::connect_easy_tls(&nick, &pass).await.unwrap();
    // split the stream
    let (read, write) = tokio::io::split(stream);

    let client = trovochat::Client::new();
    // this runs the client in a background task, giving a future you wait on
    // you should call run before you 'block'
    let done = client.run(read, write);

    // subscribe an Irc Ready event
    // GlobalUserState can also be used to 'wait' for ready
    let mut ready = client
        .dispatcher()
        .await
        .subscribe::<trovochat::events::IrcReady>();

    // 'block' until we've received an IrcReady event
    let _ready = ready.next().await.unwrap();
    // its safe to join channels after this point

    // join a channel
    client.writer().join(channel).await.unwrap();

    use trovochat::client::Status;
    match done.await {
        Ok(Status::Eof) => eprintln!("done!"),
        Ok(Status::Canceled) => eprintln!("client was stopped by user"),
        Err(err) => eprintln!("error: {}", err),
    }
}
