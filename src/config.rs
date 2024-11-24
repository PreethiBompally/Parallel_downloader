//! # Configuration for File Downloads
//!
//! This module defines the `DownloadConfig` struct, which is used to configure the parameters
//! for downloading a file, such as the URL, output file name, and the number of connections.
//!
//! ## Features
//! - Configures the file download settings
//! - Supports multiple concurrent connections

#[derive(Clone)]
pub struct DownloadConfig {
    /// The URL of the file to download.
    pub url: String,
    /// The name of the output file where the download will be saved.
    pub output_file: String,
    /// The number of concurrent connections to use.
    pub num_connections: usize,
}

impl DownloadConfig {
    /// Creates a new `DownloadConfig` instance.
    ///
    /// # Parameters
    /// - `url`: The URL of the file to download.
    /// - `output_file`: The name of the output file.
    /// - `num_connections`: The number of concurrent connections.
    ///
    /// # Returns
    /// A new `DownloadConfig` instance.
    pub fn new(url: String, output_file: String, num_connections: usize) -> Self {
        Self {
            url,
            output_file,
            num_connections,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DownloadConfig;

    #[test]
    fn test_download_config_creation() {
        let config = DownloadConfig::new(
            "https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg".to_string(),
            "output.jpg".to_string(),
            4,
        );
        assert_eq!(config.url, "https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg");
        assert_eq!(config.output_file, "output.jpg");
        assert_eq!(config.num_connections, 4);
    }
}
