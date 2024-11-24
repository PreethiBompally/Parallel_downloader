use parallel_downloader::{DownloadConfig, DownloadManager, DownloaderError, dns, connection, http};
use std::fs::{File, remove_file};
use std::io::Read;

#[test]
fn test_full_download_process() -> Result<(), DownloaderError> {
    const URL: &str = "https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg";
    const HOSTNAME: &str = "cobweb.cs.uga.edu";
    const OUTPUT_FILE: &str = "test_output.jpg";

    let ip = dns::get_request_ip(HOSTNAME)?;
    assert!(ip.is_ipv4(), "Failed to resolve a valid IPv4 address for {}", HOSTNAME);

    let mut stream = connection::establish_tls_connection(HOSTNAME, ip)?;
    assert!(stream.get_ref().peer_addr().is_ok(), "Failed to establish a TLS connection to {}", HOSTNAME);

    let (supports_range, total_size) = http::send_head_request(&mut stream, HOSTNAME, "/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg")?;
    assert!(supports_range, "Server does not support range requests");
    assert!(total_size > 0, "Received invalid file size: {}", total_size);

    let config = DownloadConfig::new(URL.to_string(), OUTPUT_FILE.to_string(), 4);
    let manager = DownloadManager::new(config, total_size);

    let result = manager.download();
    assert!(result.is_ok(), "Download process failed with error: {:?}", result);

    let mut downloaded_file = File::open(OUTPUT_FILE)?;
    let mut buffer = Vec::new();
    downloaded_file.read_to_end(&mut buffer)?;
    assert!(!buffer.is_empty(), "Downloaded file is empty");

    assert_eq!(buffer.len(), total_size as usize, "Downloaded file size does not match expected size");

    cleanup_test_files(OUTPUT_FILE, 4, ".jpg");

    Ok(())
}

fn cleanup_test_files(output_file: &str, num_parts: usize, extension: &str) {
    if let Err(e) = remove_file(output_file) {
        eprintln!("Failed to remove output file '{}': {}", output_file, e);
    }

    for i in 0..num_parts {
        let part_filename = format!("part{}{}", i, extension);
        if let Err(e) = remove_file(&part_filename) {
            eprintln!("Failed to remove part file '{}': {}", part_filename, e);
        }
    }
}

