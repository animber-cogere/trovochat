#![allow(dead_code)]
fn main() {
    use std::net::TcpStream;
    use trovochat::commands;
    use trovochat::*;

    struct Bot {
        client: Option<Client<TcpStream>>,
        writer: Writer,
    }

    impl Bot {
        pub fn new() -> Self {
            let (read, write) = TcpStream::connect(trovochat::TROVO_IRC_ADDRESS)
                .map(|w| (w.try_clone().unwrap(), w))
                .unwrap();
            let (nick, token) = trovochat::ANONYMOUS_LOGIN;
            let config = UserConfig::with_caps()
                .token(token)
                .nick(nick)
                .build()
                .unwrap();

            let client = Client::register(config, read, write).unwrap();
            let writer = client.writer();
            Self {
                client: Some(client),
                writer,
            }
        }

        pub fn write(&mut self, data: impl std::fmt::Display) {
            self.writer.command(data).unwrap();
        }

        pub fn enable_privmsg(&mut self) {
            if let Some(client) = self.client.take() {
                self.client.replace(client.filter::<commands::PrivMsg>());
            }
        }

        pub fn disable_privmsg(&mut self) {
            if let Some(client) = self.client.as_mut() {
                client.remove_filter::<commands::PrivMsg>();
            }
        }

        pub fn run(self) {
            for _ev in self.client.unwrap() {
                // stuff
                // self.write() etc
            }
        }
    }

    let mut bot = Bot::new();
    bot.enable_privmsg();
    bot.run()
}
