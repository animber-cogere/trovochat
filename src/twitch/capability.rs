/// Capability used to enable extra functionality with the protocol
///
/// Without any of these specified, you will just able to read/write basic messages
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Capability {
    /// Membership capability
    ///
    /// Read basic IRC messages from a Trovo channel allows you to see who is in the channel
    Membership,
    /// Tags capability
    ///
    /// Provides metadata attached to each message
    Tags,
    /// Commands capability
    ///
    /// Enables many Trovo specific commands
    Commands,
}

impl Capability {
    /// Encode this capability as a string, to be sent to the server
    pub fn encode_as_str(self) -> &'static str {
        match self {
            Self::Membership => "CAP REQ :trovo.tv/membership",
            Self::Tags => "CAP REQ :trovo.tv/tags",
            Self::Commands => "CAP REQ :trovo.tv/commands",
        }
    }

    /// Attempts to 'parse' this capability from a string
    ///
    /// This will take the form of `trovo.tv/$tag` and produce a `Capability`
    pub(crate) fn maybe_from_str(input: &str) -> Option<Self> {
        match input {
            "trovo.tv/membership" => Some(Self::Membership),
            "trovo.tv/tags" => Some(Self::Tags),
            "trovo.tv/commands" => Some(Self::Commands),
            _ => None,
        }
    }
}
