use std::net::{IpAddr, ToSocketAddrs};
use crate::error::DownloaderError;

pub fn get_request_ip(hostname: &str) -> Result<IpAddr, DownloaderError> {
    let socket_addr = format!("{}:443", hostname)
        .to_socket_addrs()
        .map_err(|e| DownloaderError::DnsError(e.to_string()))?
        .next()
        .ok_or_else(|| DownloaderError::DnsError("No IP address found".into()))?;
    
    Ok(socket_addr.ip())
}