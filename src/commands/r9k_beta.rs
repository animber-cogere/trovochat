use crate::Encodable;
use std::{
    borrow::Cow,
    io::{Result, Write},
};

use super::ByteWriter;

/// Enables r9k mode.    
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct R9kBeta<'a> {
    pub(crate) channel: Cow<'a, str>,
}

/// Enables r9k mode.
///
/// Use [r9k_beta_off] to disable.
///
/// [r9k_beta_off]: ./fn.r9k_beta_off.html
pub fn r9k_beta(channel: &str) -> R9kBeta<'_> {
    let channel = super::make_channel(channel);
    R9kBeta { channel }
}

impl<'a> Encodable for R9kBeta<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(&&*self.channel, &[&"/r9kbeta"])
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn r9k_beta_encode() {
        test_encode(r9k_beta("#museun"), "PRIVMSG #museun :/r9kbeta\r\n");
        test_encode(r9k_beta("museun"), "PRIVMSG #museun :/r9kbeta\r\n");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn r9k_beta_serde() {
        test_serde(r9k_beta("#museun"), "PRIVMSG #museun :/r9kbeta\r\n");
        test_serde(r9k_beta("museun"), "PRIVMSG #museun :/r9kbeta\r\n");
    }
}
