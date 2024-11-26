#![no_main]
use libfuzzer_sys::fuzz_target;
use parallel_downloader::downloader::download_part;
use parallel_downloader::dns::get_request_ip;
use parallel_downloader::connection::establish_tls_connection;
use parallel_downloader::config::DownloadConfig;

fuzz_target!(|data: &[u8]| {
    if let Ok(path) = std::str::from_utf8(data) {
        let hostname = "example.com";
        if let Ok(ip) = get_request_ip(hostname) {
            if let Ok(_stream) = establish_tls_connection(hostname, ip) {
                let part = parallel_downloader::downloader::DownloadPart {
                    start: 0,
                    end: 100,
                    part_number: 0,
                };
                let config = DownloadConfig::new(
                    "https://example.com/file.jpg".to_string(),
                    "output.jpg".to_string(),
                    4,
                );

                let _ = download_part(hostname, path, &part, &config);
            }
        }
    }
});
