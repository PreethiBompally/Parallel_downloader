//! # Download Manager
//!
//! This module orchestrates the downloading of files in parallel by splitting the file into
//! parts and downloading each part concurrently. It also merges the parts into a final file.
//!
//! ## Features
//! - Splits files into parts for parallel downloads.
//! - Manages threads for downloading each part.
//! - Merges the downloaded parts into a complete file.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use url::Url;
use crate::{config::DownloadConfig, connection, dns, error::DownloaderError};

#[derive(Clone)]
struct DownloadPart {
    /// Starting byte of the part.
    start: u64,
    /// Ending byte of the part.
    end: u64,
    /// Identifier for the part.
    part_number: usize,
}

pub struct DownloadManager {
    /// Configuration for the download.
    config: DownloadConfig,
    /// Total size of the file.
    #[allow(dead_code)]
    total_size: u64,
    /// Parts of the file to download.
    parts: Vec<DownloadPart>,
}

impl DownloadManager {
    /// Creates a new `DownloadManager` instance.
    ///
    /// # Parameters
    /// - `config`: The configuration for the download.
    /// - `total_size`: The total size of the file.
    ///
    /// # Returns
    /// A new `DownloadManager` instance.
    pub fn new(config: DownloadConfig, total_size: u64) -> Self {
        let mut parts = Vec::new();
        let part_size = total_size / config.num_connections as u64;

        for i in 0..config.num_connections {
            let start = i as u64 * part_size;
            let end = if i == config.num_connections - 1 {
                total_size - 1
            } else {
                start + part_size - 1
            };

            parts.push(DownloadPart {
                start,
                end,
                part_number: i,
            });
        }

        Self {
            config,
            total_size,
            parts,
        }
    }

    /// Downloads the file using multiple threads.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the download.
    pub fn download(&self) -> Result<(), DownloaderError> {
        let url = Url::parse(&self.config.url)?;
        let hostname = url.host_str()
            .ok_or_else(|| DownloaderError::UrlParseError(url::ParseError::EmptyHost))?;
        let path = url.path();

        let downloaded_parts = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for part in &self.parts {
            let part = part.clone();
            let hostname = hostname.to_string();
            let path = path.to_string();
            let downloaded_parts = Arc::clone(&downloaded_parts);
            let config = self.config.clone();
            let part_filename = self.get_part_filename(part.part_number);

            let handle = thread::spawn(move || {
                let data = download_part(&hostname, &path, &part, &config)?;
                if data.is_empty() {
                    return Err(DownloaderError::ResponseError("Received empty response body".into()));
                }

                let mut part_file = File::create(&part_filename)?;
                part_file.write_all(&data)?;

                let mut parts = downloaded_parts.lock().unwrap();
                parts.push((part.part_number, data));
                Ok::<(), DownloaderError>(())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()?;
        }

        self.merge_parts()?;
        Ok(())
    }

    /// Merges the downloaded parts into a single file.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the merge.
    pub fn merge_parts(&self) -> Result<(), DownloaderError> {
        if self.parts.is_empty() {
            return Err(DownloaderError::FileError(
                "No parts to merge. Parts list is empty.".into(),
            ));
        }

        if let Some(parent) = Path::new(&self.config.output_file).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut output_file = File::create(&self.config.output_file)?;
        for part_number in 0..self.parts.len() {
            let part_filename = self.get_part_filename(part_number);
            let mut part_file = File::open(&part_filename)?;
            let mut buffer = Vec::new();
            part_file.read_to_end(&mut buffer)?;
            output_file.write_all(&buffer)?;
        }

        Ok(())
    }

    fn get_part_filename(&self, part_number: usize) -> String {
        let base_path = Path::new(&self.config.output_file);
        let parent = base_path.parent().unwrap_or_else(|| Path::new(""));
        let extension = base_path.extension().unwrap_or_default();
        
        let part_name = format!("part{}", part_number);
        parent.join(part_name)
            .with_extension(extension)
            .to_string_lossy()
            .into_owned()
    }
}

fn download_part(
    hostname: &str,
    path: &str,
    part: &DownloadPart,
    _config: &DownloadConfig,
) -> Result<Vec<u8>, DownloaderError> {
    let ip = dns::get_request_ip(hostname)?;
    let mut stream = connection::establish_tls_connection(hostname, ip)?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nRange: bytes={}-{}\r\nUser-Agent: rust-downloader/1.0\r\n\r\n",
        path, hostname, part.start, part.end
    );

    stream.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    if let Some(pos) = find_body_start(&response) {
        Ok(response[pos..].to_vec())
    } else {
        Err(DownloaderError::ResponseError("Could not find response body".into()))
    }
}

fn find_body_start(response: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < response.len() - 3 {
        if &response[i..i + 4] == b"\r\n\r\n" {
            return Some(i + 4);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{DownloadManager, DownloadConfig};

    #[test]
    fn test_download_manager_creation() {
        let config = DownloadConfig::new(
            "https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg".to_string(),
            "output.jpg".to_string(),
            4,
        );
        let manager = DownloadManager::new(config.clone(), 1000);
        assert_eq!(manager.config.url, config.url);
        assert_eq!(manager.config.output_file, config.output_file);
        assert_eq!(manager.config.num_connections, config.num_connections);
    }

    #[test]
    fn test_merge_parts_empty() {
        use std::fs;
        let config = DownloadConfig::new(
            "https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg".to_string(),
            "output.jpg".to_string(),
            4,
        );

        let mut manager = DownloadManager::new(config, 1000);
        manager.parts.clear();
        manager.parts.iter().for_each(|part| {
            let filename = manager.get_part_filename(part.part_number);
            let _ = fs::remove_file(filename);
        });
        let result = manager.merge_parts();
        assert!(result.is_err(), "Expected merge_parts to fail when no parts exist.");
    }
}
