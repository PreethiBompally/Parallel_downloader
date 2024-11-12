pub mod config;
pub mod connection;
pub mod dns;
pub mod downloader;
pub mod error;
pub mod http;
pub mod tcp;

pub use config::DownloadConfig;
pub use downloader::DownloadManager;
pub use error::DownloaderError;

pub use downloader::{DownloadManager, download_file};