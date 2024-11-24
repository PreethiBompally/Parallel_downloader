//! # DNS Resolution
//!
//! This module resolves hostnames into their corresponding IP addresses. It is a critical
//! component of the downloader that ensures the correct server is contacted.
//!
//! ## Features
//! - Resolves a hostname to an IP address.
//! - Handles errors gracefully.

use std::net::{IpAddr, ToSocketAddrs};
use crate::error::DownloaderError;

/// Resolves the given hostname to an IP address.
///
/// # Parameters
/// - `hostname`: The hostname to resolve.
///
/// # Returns
/// The resolved IP address as an `IpAddr`.
pub fn get_request_ip(hostname: &str) -> Result<IpAddr, DownloaderError> {
    if let Ok(ip) = hostname.parse::<IpAddr>() {
        return Ok(ip);
    }

    let socket_addr = format!("{}:443", hostname)
        .to_socket_addrs()
        .map_err(|e| DownloaderError::DnsError(e.to_string()))?
        .next()
        .ok_or_else(|| DownloaderError::DnsError("No IP address found".into()))?;
    Ok(socket_addr.ip())
}

#[cfg(test)]
mod tests {
    use super::get_request_ip;
    use std::net::IpAddr;

    #[test]
    fn test_valid_hostname() {
        let ip = get_request_ip("example.com").unwrap();
        assert!(matches!(ip, IpAddr::V4(_) | IpAddr::V6(_)));
    }

    #[test]
    fn test_invalid_hostname() {
        let result = get_request_ip("invalid-hostname");
        assert!(result.is_err());
    }
}
