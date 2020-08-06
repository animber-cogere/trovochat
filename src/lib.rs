// #![warn(
//     missing_docs,
//     missing_debug_implementations,
//     missing_copy_implementations,
//     trivial_casts,
//     trivial_numeric_casts,
//     unsafe_code,
//     unstable_features,
//     unused_import_braces,
//     unused_qualifications
// )]
// #![cfg_attr(docsrs, feature(doc_cfg))]
// /*!
// This crate provides a way to interface with [Trovo]'s chat.

// Along with the messages as Rust types, it provides methods for sending messages.

// # Demonstration
// See `examples/demo.rs` for a demo of the api

// ---
// Here's a quick link to the [Event Mapping](./struct.Dispatcher.html#a-table-of-mappings)

// [Trovo]: https://www.trovo.tv
// */
// #[cfg(all(doctest, feature = "async", feature = "tokio_native_tls"))]
// doc_comment::doctest!("../README.md");

pub mod trovo;
pub use trovo::*;

pub mod rate_limit;

/// The Trovo IRC address for non-TLS connections
pub const TROVO_IRC_ADDRESS: &str = "irc.chat.trovo.tv:6667";

/// The Trovo IRC address for TLS connections
pub const TROVO_IRC_ADDRESS_TLS: &str = "irc.chat.trovo.tv:6697";

/// The Trovo WebSocket address for non-TLS connections
pub const TROVO_WS_ADDRESS: &str = "ws://irc-ws.chat.trovo.tv:80";

/// The Trovo WebSocket address for TLS connections
pub const TROVO_WS_ADDRESS_TLS: &str = "wss://irc-ws.chat.trovo.tv:443";

// TODO WIP
pub mod ng;
