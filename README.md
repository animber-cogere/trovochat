# trovochat
[![Documentation][trovochat_docs_badge]][trovochat_docs]
[![Crates][trovochat_crates_badge]][trovochat_crates]
[![Actions][actions_badge]][actions]

This crate provides a way to interact with [Trovo]'s chat.

Along with parse messages as Rust types, it provides methods for sending messages.

See its [README](trovochat/README.md)

# simple_event_map
[![Documentation][sem_docs_badge]][sem_docs]
[![Crates][sem_crates_badge]][sem_crates]
[![Actions][actions_badge]][actions]

This crate provides a simple event mapping which provides listeners as a blocking iterator or an asynchronous stream.

This primarily used in trovochat, but included in the workspace so things are updated together

See its [README](simple_event_map/README.md)

# License for all crate
`trovochat` and `simple_event_map` are primarily distributed under the terms of both the **MIT license** and the **Apache License (Version 2.0)**.

See [LICENSE-APACHE][APACHE] and [LICENSE-MIT][MIT] for details.

[actions_badge]: https://github.com/museun/trovochat/workflows/Rust/badge.svg
[actions]: https://github.com/museun/trovochat/actions

[trovochat_docs_badge]: https://docs.rs/trovochat/badge.svg
[trovochat_docs]: https://docs.rs/trovochat
[trovochat_crates_badge]: https://img.shields.io/crates/v/trovochat.svg
[trovochat_crates]: https://crates.io/crates/trovochat

[sem_docs_badge]: https://docs.rs/simple_event_map/badge.svg
[sem_docs]: https://docs.rs/simple_event_map
[sem_crates_badge]: https://img.shields.io/crates/v/simple_event_map.svg
[sem_crates]: https://crates.io/crates/simple_event_map

[APACHE]: ./LICENSE-APACHE
[MIT]: ./LICENSE-MIT
[Trovo]: https://dev.trovo.tv
