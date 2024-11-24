#![no_main]
use libfuzzer_sys::fuzz_target;
use parallel_downloader::{DownloadConfig, DownloadManager};
use std::fs::{File};
use std::io::Write;

fuzz_target!(|data: &[u8]| {
    let output_file = "test_fuzz_output.jpg";
    let config = DownloadConfig::new(
        "https://example.com/file.jpg".to_string(),
        output_file.to_string(),
        4,
    );

    let manager = DownloadManager::new(config, 1000);

    for i in 0..4 {
        let filename = format!("part{}.jpg", i);
        let mut part_file = File::create(&filename).unwrap();
        part_file.write_all(data).unwrap();
    }

    let _ = manager.merge_parts();

    for i in 0..4 {
        let filename = format!("part{}.jpg", i);
        let _ = std::fs::remove_file(&filename);
    }
});
