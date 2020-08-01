use crate::ng::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct EmoteOnlyOff<'a> {
    pub channel: &'a str,
}

pub fn emote_only_off(channel: &str) -> EmoteOnlyOff<'_> {
    EmoteOnlyOff { channel }
}

impl<'a> Encodable for EmoteOnlyOff<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(self.channel, &[&"/emoteonlyoff"])
    }
}