use super::{IrcMessage, Str, StrIndex};
use crate::ng::{FromIrcMessage, InvalidMessage, Validator};

/// A parsed Capability
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Capability<'t> {
    // This Capability was Acknowledged
    Acknowledged(
        /// The name of the requested capability
        &'t str,
    ),
    // This Capability was not acknowledged
    NotAcknowledged(
        /// The name of the requested capability
        &'t str,
    ),
}

/// Acknowledgement (or not) on a **CAPS** request
#[derive(Debug, Clone, PartialEq)]
pub struct Cap<'t> {
    raw: Str<'t>,
    capability: StrIndex,
    acknowledged: bool,
}

impl<'t> Cap<'t> {
    raw!();

    /// The parsed capability
    pub fn capability(&self) -> Capability<'_> {
        let cap = &self.raw[self.capability];
        if self.acknowledged {
            Capability::Acknowledged(cap)
        } else {
            Capability::NotAcknowledged(cap)
        }
    }
}

impl<'a> FromIrcMessage<'a> for Cap<'a> {
    type Error = InvalidMessage;

    fn from_irc(msg: IrcMessage<'a>) -> Result<Self, Self::Error> {
        const ACK: &str = "ACK";

        msg.expect_command(IrcMessage::CAP)?;

        let this = Self {
            capability: msg.expect_data_index()?,
            acknowledged: msg.expect_arg(1)? == ACK,
            raw: msg.raw,
        };

        Ok(this)
    }
}

serde_struct!(Cap { raw, capability });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ng::irc;

    #[test]
    #[cfg(feature = "serde")]
    fn cap_serde() {
        let input = ":tmi.trovo.tv CAP * ACK :trovo.tv/membership\r\n";
        crate::ng::serde::round_trip_json::<Cap>(input);
    }

    #[test]
    fn cap_acknowledged() {
        let input = ":tmi.trovo.tv CAP * ACK :trovo.tv/membership\r\n\
                     :tmi.trovo.tv CAP * ACK :trovo.tv/tags\r\n\
                     :tmi.trovo.tv CAP * ACK :trovo.tv/commands\r\n";
        let expected = &[
            "trovo.tv/membership",
            "trovo.tv/tags",
            "trovo.tv/commands",
        ];
        for (msg, expected) in irc::parse(&input).map(|s| s.unwrap()).zip(expected) {
            let msg = Cap::from_irc(msg).unwrap();
            assert_eq!(msg.capability(), Capability::Acknowledged(*expected));
        }
    }

    #[test]
    fn cap_failed() {
        let input = ":tmi.trovo.tv CAP * NAK :foobar\r\n";
        for msg in irc::parse(input).map(|s| s.unwrap()) {
            let cap = Cap::from_irc(msg).unwrap();
            assert_eq!(cap.capability(), Capability::NotAcknowledged("foobar"));
        }
    }
}
