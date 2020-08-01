use crate::ng::Encodable;
use std::{

    io::{Result, Write},
};

use super::ByteWriter;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]


#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Unraid<'a> {
    pub 
    channel: &'a str,
}

pub fn unraid(channel: &str) -> Unraid<'_> {
    Unraid { channel }
}

impl<'a> Encodable for Unraid<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(self.channel, &[&"/unraid"])
    }
}