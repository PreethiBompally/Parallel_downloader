//! # HTTP Request Handling
//!
//! This module provides functionality to handle HTTP requests, specifically
//! `HEAD` requests, to retrieve metadata such as file size and support for range-based downloads.
//!
//! ## Features
//! - Sends `HEAD` requests to check file details.
//! - Parses response headers for content-length and range support.

use std::io::{Read, Write};
use native_tls::TlsStream;
use std::net::TcpStream;
use crate::error::DownloaderError;

/// Sends a `HEAD` request to the specified path and retrieves metadata.
///
/// # Parameters
/// - `stream`: The TLS stream to the server.
/// - `hostname`: The hostname of the server.
/// - `path`: The file path on the server.
///
/// # Returns
/// A tuple containing:
/// - A boolean indicating if range requests are supported.
/// - The total size of the file.
///
/// # Errors
/// Returns a `DownloaderError` if the request fails or the response is invalid.

pub fn send_head_request(
    stream: &mut TlsStream<TcpStream>,
    hostname: &str,
    path: &str,
) -> Result<(bool, u64), DownloaderError> {
    let request = format!(
        "HEAD {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: rust-downloader/1.0\r\n\r\n",
        path, hostname
    );

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let supports_range = response.contains("Accept-Ranges: bytes");
    let content_length = parse_content_length(&response)?;

    Ok((supports_range, content_length))
}

/// Parses the `Content-Length` header from an HTTP response.
///
/// # Parameters
/// - `response`: The raw HTTP response as a string.
///
/// # Returns
/// The file size as a `u64`.
///
/// # Errors
/// Returns a `DownloaderError` if the `Content-Length` header is missing or invalid.
fn parse_content_length(response: &str) -> Result<u64, DownloaderError> {
    for line in response.lines() {
        if line.to_lowercase().starts_with("content-length:") {
            return line
                .split(':')
                .nth(1)
                .and_then(|s| s.trim().parse().ok())
                .ok_or_else(|| {
                    DownloaderError::ResponseError("Invalid Content-Length header".into())
                });
        }
    }
    Err(DownloaderError::ResponseError(
        "Content-Length header not found".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::send_head_request;
    use std::net::TcpStream;
    use native_tls::TlsConnector;

    #[test]
    fn test_send_head_request_valid() {
        let stream_result = TcpStream::connect("93.184.216.34:443");
        assert!(stream_result.is_ok(), "Expected TcpStream connection to succeed");

        if let Ok(stream) = stream_result {
            let connector = TlsConnector::new().unwrap();
            let mut tls_stream = connector.connect("example.com", stream).unwrap();
            let result = send_head_request(&mut tls_stream, "example.com", "/");
            assert!(result.is_ok(), "Expected HEAD request to succeed");
        }
    }

    #[test]
    fn test_send_head_request_invalid() {
        let stream_result = TcpStream::connect("127.0.0.1:443");
        assert!(stream_result.is_err(), "Expected TcpStream connection to fail");
    }
}