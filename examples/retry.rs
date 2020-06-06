use tokio::stream::StreamExt as _;

// your trovo name. it should be associated with your oauth token
fn get_nick() -> String {
    std::env::var("TROVO_NICK").unwrap()
}

// your oauth token
fn get_pass() -> String {
    std::env::var("TROVO_PASS").unwrap()
}

// a channel to join
fn get_channel() -> String {
    std::env::var("TROVO_CHANNEL").unwrap()
}

#[tokio::main]
async fn main() {
    // make a dispatcher. this is how you subscribe to events (streams of 'commands' from trovo)
    let dispatcher = trovochat::Dispatcher::new();

    // make a new runner with the dispatcher. this is the main loop + some types for interacting with it
    let (mut runner, mut control) = trovochat::Runner::new(dispatcher.clone());

    // create a connector -- this is a connection factory. this is called on each new connection
    let connector = trovochat::Connector::new(|| async move {
        let (nick, pass) = (get_nick(), get_pass());
        trovochat::native_tls::connect_easy(&nick, &pass).await
    });

    // start the runner and instruct to reconnect immediately on any failure
    // if control.abort() was used, it'll cancel the reconnection loop
    let done = tokio::task::spawn(async move {
        runner
            .run_with_retry(connector, trovochat::RetryStrategy::immediately)
            .await
    });

    // subscribe to some events
    let mut raw = dispatcher.subscribe::<trovochat::events::Raw>();

    // loop over the EventStream. when the Runner quits (or the dispatcher is
    // dropped, or you manually clear its subscriptions) this stream will end
    let handle = tokio::spawn(async move {
        while let Some(msg) = raw.next().await {
            // the raw line from the connection
            eprintln!("{}", msg.raw.escape_debug());
        }
        eprintln!("done with the 'all' loop")
    });

    // this will loop forever
    tokio::spawn(async move {
        loop {
            eprintln!("!! waiting for ready");
            // wait for the connection to be ready
            let _ready = dispatcher
                .wait_for::<trovochat::events::IrcReady>()
                .await
                .unwrap();

            // and then join a channel
            control.writer().join(get_channel()).await.unwrap();

            // and then after 3 seconds bail
            tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
            control.writer().raw("quit :bye").await.unwrap();

            // wait for the reconnection to happen
            control.wait_for_reconnect().await;
            eprintln!("we've reconnected");

            // and we'll start over again, waiting for the connection to be ready
            eprintln!("!! Lets go again");
        }
    });

    // wait for the 'runner' task to join back in
    // the unwrap is for any panics in the task
    match done.await.unwrap() {
        Ok(..) => eprintln!("done!"),
        Err(err) => eprintln!("error: {}", err),
    }

    // and you can also join your event stream tasks. they'll return the Runner is done
    handle.await.unwrap()
}
