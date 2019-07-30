/// Capabilities allow you to get more data from trovo
///
/// The default, `generic` is very simplistic (basically just read/write PRIVMSGs for a channel)
///
/// While enabling `membership` + `commands` + `tags` will allow you to get a much more rich set of messages
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Capability {
    /// Generic capability, the default.
    ///
    /// Simply read basic irc messages from a trovo channel
    Generic,
    /// Membership capability
    ///
    /// Similar to Generic, but allows to see who is in the channel
    Membership,
    /// Commands capability
    ///
    /// Enables many trovo specific commands
    Commands,
    /// Tags capability
    ///
    /// Provides metadata attached to each message
    Tags,
    // Reserve the right to add more fields to this enum
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Capability {
    pub(crate) fn get_command(self) -> Option<&'static str> {
        match self {
            Capability::Generic => None,
            Capability::Membership => Some("CAP REQ :trovo.tv/membership"),
            Capability::Commands => Some("CAP REQ :trovo.tv/commands"),
            Capability::Tags => Some("CAP REQ :trovo.tv/tags"),
            Capability::__Nonexhaustive => unreachable!(),
        }
    }
}

impl Default for Capability {
    /// The default capability is `Generic`
    fn default() -> Self {
        Capability::Generic
    }
}
