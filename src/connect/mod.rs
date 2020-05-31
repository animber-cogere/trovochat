use crate::UserConfig;

type Stream = tokio::net::TcpStream;

/// Connect to Trovo without TLS. Using the provided [`UserConfig`][UserConfig].
///
/// This registers with the connection before returning it.
///
/// To connect using TLS:
///
/// enable one of:
/// * `tokio_rustls`
/// * `tokio_native_tls`
///
/// and then use the respective:
/// * [`trovochat::native_tls::connect`][native_tls_connect]
/// * [`trovochat::rustls::connect`][rustls_tls_connect]
///
/// [native_tls_connect]: ./native_tls/fn.connect.html
/// [rustls_tls_connect]: ./rustls/fn.connect.html
/// [UserConfig]: ./struct.UserConfig.html
///
/// # Example
/// ```rust,no_run
/// # use trovochat::*;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
/// let user_config = UserConfig::builder().anonymous().build()?;
/// let mut stream = trovochat::connect(&user_config).await?;
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// # }).unwrap();
/// ```
pub async fn connect(config: &UserConfig) -> std::io::Result<Stream> {
    let mut stream = tokio::net::TcpStream::connect(crate::TROVO_IRC_ADDRESS_TLS).await?;
    crate::register(config, &mut stream).await?;
    Ok(stream)
}

/// Connect to Trovo without TLS. Using the provided `name`, `token`.
///
/// This registers with the connection before returning it.
///
/// To connect using TLS:
///
/// enable one of:
/// * `tokio_rustls`
/// * `tokio_native_tls`
///
/// and then use the respective:
/// * [`trovochat::native_tls::connect`][native_tls_connect]
/// * [`trovochat::rustls::connect`][rustls_tls_connect]
///
/// [native_tls_connect]: ./native_tls/fn.connect.html
/// [rustls_tls_connect]: ./rustls/fn.connect.html
///
/// # Example
/// ```rust,no_run
/// # use trovochat::*;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
/// let (name, token) = ANONYMOUS_LOGIN;
/// let mut stream = trovochat::connect_easy(&name, &token).await?;
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// # }).unwrap();
/// ```
pub async fn connect_easy(name: &str, token: &str) -> std::io::Result<Stream> {
    use std::io::{Error, ErrorKind};

    let mut stream = tokio::net::TcpStream::connect(crate::TROVO_IRC_ADDRESS_TLS).await?;

    let config = crate::simple_user_config(name, token) //
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    crate::register(&config, &mut stream).await?;

    Ok(stream)
}
