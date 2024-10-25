use std::fs::File;
use std::io::{Read, Write};
use crate::config::DownloadConfig;
use crate::error::DownloaderError;

pub struct DownloadManager {
    config: DownloadConfig,
    total_size: u64,
}

impl DownloadManager {
    pub fn new(config: DownloadConfig, total_size: u64) -> Self {
        Self {
            config,
            total_size,
        }
    }

    pub fn download(&self, mut stream: impl Read) -> Result<(), DownloaderError> {
        println!("Downloading...");
        let mut file = File::create(&self.config.output_file)
            .map_err(|e| DownloaderError::FileError(format!("Failed to create output file: {}", e)))?;
    
        let mut buffer = vec![0; 1024 * 1024];
        let mut downloaded_size = 0;
    
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[0..bytes_read])
                .map_err(|e| DownloaderError::FileError(format!("Error writing to file: {}", e)))?;
            downloaded_size += bytes_read as u64;
    
            let progress = (downloaded_size as f64 / self.total_size as f64) * 100.0;
            println!("Downloaded: {:.2}%", progress);
        }
    
        // if downloaded_size != self.total_size {
        //     return Err(DownloaderError::FileError("Downloaded size does not match expected".into()));
        // }
    
        println!("\nDownload completed successfully!");
        Ok(())
    }
}
