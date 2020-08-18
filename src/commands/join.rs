use crate::Encodable;
use std::{
    borrow::Cow,
    io::{Result, Write},
};

use super::ByteWriter;

/// Join a channel. This handles prepending a leading '#' for you if you omit it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Join<'a> {
    pub(crate) channel: Cow<'a, str>,
}

/// Join a channel. This handles prepending a leading '#' for you if you omit it.
pub fn join(channel: &str) -> Join<'_> {
    let channel = super::make_channel(channel);
    Join { channel }
}

impl<'a> Encodable for Join<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).parts(&[&"JOIN", &&*self.channel])
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn join_encode() {
        test_encode(join("#museun"), "JOIN #museun\r\n");
        test_encode(join("museun"), "JOIN #museun\r\n");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn join_serde() {
        test_serde(join("#museun"), "JOIN #museun\r\n");
        test_serde(join("museun"), "JOIN #museun\r\n");
    }
}
