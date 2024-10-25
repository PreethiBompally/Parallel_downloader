use std::io::{Read, Write};
use native_tls::TlsStream;
use std::net::TcpStream;
use crate::error::DownloaderError;

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