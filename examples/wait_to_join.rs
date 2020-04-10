/* in your Cargo.toml
[dependencies]
trovochat = "0.10"                                        # this crate
tokio = { version = "0.2", features = ["full", "macros"] } # you need tokio to run it
*/

#[tokio::main]
async fn main() {
    // make a new dispatcher
    let dispatcher = trovochat::Dispatcher::new();

    let (nick, pass, channel) = (
        std::env::var("TROVO_NICK").unwrap(),
        std::env::var("TROVO_PASS").unwrap(),
        std::env::var("TROVO_CHANNEL").unwrap(),
    );

    // give dispatcher to the client, also defaulting the abort signal
    let (runner, mut control) =
        trovochat::Runner::new(dispatcher.clone(), trovochat::RateLimit::default());

    let stream = trovochat::connect_easy_tls(&nick, &pass).await.unwrap();

    // this runs the client in a background task, giving a future you wait on
    // you should call run before you 'block'
    let done = tokio::task::spawn(runner.run(stream));

    // subscribe an Irc Ready event
    // GlobalUserState can also be used to 'wait' for ready
    // 'block' until we've received an IrcReady event
    let _ready = dispatcher
        .wait_for::<trovochat::events::IrcReady>()
        .await
        .unwrap();

    // its safe to join channels after this point

    // join a channel
    control.writer().join(channel).await.unwrap();

    use trovochat::Status;
    match done.await.unwrap() {
        Ok(Status::Eof) => eprintln!("done!"),
        Ok(Status::Canceled) => eprintln!("client was stopped by user"),
        Ok(Status::Timeout) => eprintln!("client connection timed out"),
        Err(err) => eprintln!("error: {}", err),
    }
}
