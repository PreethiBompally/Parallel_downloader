use std::io;
use url;
use native_tls::HandshakeError;
use std::net::TcpStream;
use std::fmt;

#[derive(Debug)]
pub enum DownloaderError {
    IoError(io::Error),
    TlsError(native_tls::Error),
    TlsHandshakeError(String),
    UrlParseError(url::ParseError),
    DnsError(String),
    FileError(String),
    ConnectionError(String),
    ResponseError(String),
    UserInputError(String),
}

impl fmt::Display for DownloaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloaderError::IoError(e) => write!(f, "IO error: {}", e),
            DownloaderError::TlsError(e) => write!(f, "TLS error: {}", e),
            DownloaderError::TlsHandshakeError(e) => write!(f, "TLS handshake error: {}", e),
            DownloaderError::UrlParseError(e) => write!(f, "URL parse error: {}", e),
            DownloaderError::DnsError(e) => write!(f, "DNS error: {}", e),
            DownloaderError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            DownloaderError::ResponseError(e) => write!(f, "Server response error: {}", e),
            DownloaderError::UserInputError(e) => write!(f, "Invalid input: {}", e),
            DownloaderError::FileError(e) => write!(f, "File error: {}", e),
        }
    }
}

impl std::error::Error for DownloaderError {}

impl From<io::Error> for DownloaderError {
    fn from(error: io::Error) -> Self {
        DownloaderError::IoError(error)
    }
}

impl From<native_tls::Error> for DownloaderError {
    fn from(error: native_tls::Error) -> Self {
        DownloaderError::TlsError(error)
    }
}

impl From<HandshakeError<TcpStream>> for DownloaderError {
    fn from(error: HandshakeError<TcpStream>) -> Self {
        DownloaderError::TlsHandshakeError(error.to_string())
    }
}

impl From<url::ParseError> for DownloaderError {
    fn from(error: url::ParseError) -> Self {
        DownloaderError::UrlParseError(error)
    }
}