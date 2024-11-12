use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use url::Url;
use crate::{config::DownloadConfig, connection, dns, error::DownloaderError};

#[derive(Clone)]
struct DownloadPart {
    start: u64,
    end: u64,
    part_number: usize,
}

pub struct DownloadManager {
    config: DownloadConfig,
    #[allow(dead_code)]
    total_size: u64,
    parts: Vec<DownloadPart>,
}

impl DownloadManager {
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

    fn merge_parts(&self) -> Result<(), DownloaderError> {
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
        // let file_stem = base_path.file_stem().unwrap_or_default();
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