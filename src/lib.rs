#![allow(dead_code)]
/// IRC-related stuff
pub mod irc;

/// Types associated with trovo
pub mod trovo;

#[cfg(feature = "teststream")]
mod teststream;

#[cfg(feature = "teststream")]
pub use teststream::TestStream;
