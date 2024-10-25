use std::net::{IpAddr, TcpStream};
use native_tls::{TlsConnector, TlsStream, HandshakeError};
use crate::error::DownloaderError;
use crate::tcp;

pub fn establish_tls_connection(
    hostname: &str,
    ip: IpAddr,
) -> Result<TlsStream<TcpStream>, DownloaderError> {
    // Create TCP connection
    let tcp_stream = tcp::establish_tcp_socket(ip, 443)?;

    // Create a TLS connector builder with custom settings
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| DownloaderError::TlsError(e))?;

    // Establish TLS connection
    connector
        .connect(hostname, tcp_stream)
        .map_err(|e| match e {
            HandshakeError::Failure(e) => DownloaderError::TlsError(e),
            HandshakeError::WouldBlock(_) => {
                DownloaderError::ConnectionError("TLS handshake would block".to_string())
            }
        })
}