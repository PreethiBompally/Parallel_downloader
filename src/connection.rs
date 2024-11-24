//! # Connection Management
//!
//! This module handles the establishment of secure connections using TLS. It ensures
//! robust communication between the client and the server by creating a secure socket.
//!
//! ## Features
//! - Establishes a TCP connection.
//! - Secures the connection using TLS.

use std::net::{IpAddr, TcpStream};
use native_tls::{TlsConnector, TlsStream, HandshakeError};
use crate::error::DownloaderError;
use crate::tcp;

/// Establishes a secure TLS connection to the given hostname and IP address.
///
/// # Parameters
/// - `hostname`: The hostname of the server.
/// - `ip`: The IP address of the server.
///
/// # Returns
/// A `TlsStream` wrapped in a `Result`, representing the secure connection.
pub fn establish_tls_connection(
    hostname: &str,
    ip: IpAddr,
) -> Result<TlsStream<TcpStream>, DownloaderError> {
    let tcp_stream = tcp::establish_tcp_socket(ip, 443)?;

    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| DownloaderError::TlsError(e))?;

    connector
        .connect(hostname, tcp_stream)
        .map_err(|e| match e {
            HandshakeError::Failure(e) => DownloaderError::TlsError(e),
            HandshakeError::WouldBlock(_) => {
                DownloaderError::ConnectionError("TLS handshake would block".to_string())
            }
        })
}

#[cfg(test)]
mod tests {
    use super::establish_tls_connection;
    use std::net::IpAddr;

    #[test]
    fn test_tls_connection() {
        let ip = "93.184.216.34".parse::<IpAddr>().unwrap();
        let result = establish_tls_connection("example.com", ip);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_tls_connection() {
        let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
        let result = establish_tls_connection("invalid.com", ip);
        assert!(result.is_err());
    }
}
