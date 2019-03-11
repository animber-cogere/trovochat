/// Configuration used to complete the 'registration' with the irc server
pub struct UserConfig {
    /// OAuth token from trovo, it must have the
    /// [scopes](https://dev.trovo.tv/docs/authentication/#scopes):
    /// `chat:read`, `chat:edit`
    pub token: String,
    /// Username to use on trovo. (must be associated with the oauth token)
    pub nick: String,
    //
    // TODO allow for TLS configuration here
    //
    /// Which capabilites to enable
    pub caps: Vec<crate::trovo::Capability>,
}
