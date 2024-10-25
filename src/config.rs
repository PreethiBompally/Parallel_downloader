#[derive(Clone)]
pub struct DownloadConfig {
    pub url: String,
    pub output_file: String,
    pub num_connections: usize,
}

impl DownloadConfig {
    pub fn new(url: String, output_file: String, num_connections: usize) -> Self {
        Self {
            url,
            output_file,
            num_connections,
        }
    }
}