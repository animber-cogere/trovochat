/*! [`tokio-native-tls`][native_tls] connector for using a TLS connection with Trovo.

[native_tls]: https://docs.rs/tokio-native-tls/0.1.0/tokio_native_tls/
*/
use crate::UserConfig;
type Stream = tokio_native_tls::TlsStream<tokio::net::TcpStream>;

const TROVO_DOMAIN: &str = "irc.chat.trovo.tv";

/// Connect to Trovo using TLS via **native_tls**. Using the provided [`UserConfig`][UserConfig].
///
/// This registers with the connection before returning it.
///
/// [UserConfig]: ../struct.UserConfig.html
///
/// # Example
/// ```rust,no_run
/// # use trovochat::*;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
/// let user_config = UserConfig::builder().anonymous().build()?;
/// let mut stream = trovochat::native_tls::connect(&user_config).await?;
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// # }).unwrap();
/// ```
pub async fn connect(config: &UserConfig) -> std::io::Result<Stream> {
    use std::io::{Error, ErrorKind};

    let conn: tokio_native_tls::TlsConnector = native_tls::TlsConnector::new()
        .map_err(|err| Error::new(ErrorKind::Other, err))?
        .into();

    let stream = tokio::net::TcpStream::connect(crate::TROVO_IRC_ADDRESS_TLS).await?;
    let mut stream = conn
        .connect(TROVO_DOMAIN, stream)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    crate::register(config, &mut stream).await?;

    Ok(stream)
}

/// Connect to Trovo using TLS via **native_tls**. Using the provided `name`, `token`.
///
/// This registers with the connection before returning it.
///
/// # Example
/// ```rust,no_run
/// # use trovochat::*;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
/// let (name, token) = ANONYMOUS_LOGIN;
/// let mut stream = trovochat::native_tls::connect_easy(&name, &token).await?;
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// # }).unwrap();
/// ```
pub async fn connect_easy(name: &str, token: &str) -> std::io::Result<Stream> {
    use std::io::{Error, ErrorKind};

    let conn: tokio_native_tls::TlsConnector = native_tls::TlsConnector::new()
        .map_err(|err| Error::new(ErrorKind::Other, err))?
        .into();

    let stream = tokio::net::TcpStream::connect(crate::TROVO_IRC_ADDRESS_TLS).await?;
    let mut stream = conn
        .connect(TROVO_DOMAIN, stream)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    let config = crate::simple_user_config(name, token) //
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    crate::register(&config, &mut stream).await?;

    Ok(stream)
}
