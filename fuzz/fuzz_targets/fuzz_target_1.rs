#![no_main]
use libfuzzer_sys::fuzz_target;
use parallel_downloader::dns::get_request_ip;

fuzz_target!(|data: &[u8]| {
    if let Ok(hostname) = std::str::from_utf8(data) {
        let _ = get_request_ip(hostname);
    }
});
