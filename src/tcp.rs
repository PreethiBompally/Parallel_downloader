//! # TCP Socket Handling
//!
//! This module provides utilities for establishing TCP connections.
use crate::error::DownloaderError;
use std::net::{TcpStream, IpAddr, SocketAddr};

/// Establishes a TCP socket connection to the given IP address and port.
///
/// # Parameters
/// - `ip_address`: The IP address to connect to.
/// - `port`: The port number.
///
/// # Returns
/// A `TcpStream` wrapped in a `Result`.
 
pub fn establish_tcp_socket(ip_address: IpAddr, port: u16) -> Result<TcpStream, DownloaderError> {
    let socket_addr = SocketAddr::new(ip_address, port);
    TcpStream::connect(socket_addr)
        .map_err(|e| DownloaderError::ConnectionError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::establish_tcp_socket;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_valid_tcp_connection() {
        let ip = IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34));
        let result = establish_tcp_socket(ip, 443);
        assert!(result.is_ok(), "Expected a valid TCP connection, but got an error: {:?}", result.err());
    }
}
