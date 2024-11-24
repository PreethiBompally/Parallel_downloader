#![no_main]
use libfuzzer_sys::fuzz_target;
use parallel_downloader::http::send_head_request;
use native_tls::{TlsConnector};
use std::net::TcpStream;

fuzz_target!(|data: &[u8]| {
    if let Ok(path) = std::str::from_utf8(data) {
        let hostname = "example.com";
        if let Ok(stream) = TcpStream::connect("93.184.216.34:443") {
            let connector = TlsConnector::new().unwrap();
            if let Ok(mut tls_stream) = connector.connect(hostname, stream) {
                let _ = send_head_request(&mut tls_stream, hostname, path);
            }
        }
    }
});
