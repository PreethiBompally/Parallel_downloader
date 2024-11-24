use parallel_downloader::{DownloadConfig, DownloadManager, DownloaderError,dns, connection, http};
use url::Url;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), DownloaderError> {
    println!("\n=== Parallel File Downloader ===\n");

    loop {
        println!("\nOptions:");
        println!("1. Download a file");
        println!("2. Exit");
        print!("\nEnter your choice (1-2): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                if let Err(e) = handle_download() {
                    eprintln!("\nError: {}", e);
                }
            }
            "2" => {
                println!("\nThank You!");
                break;
            }
            _ => println!("\nInvalid choice. Please try again."),
        }
    }

    Ok(())
}

fn handle_download() -> Result<(), DownloaderError> {
    // Get URL
    print!("\nEnter URL to download: ");
    io::stdout().flush().unwrap();
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?;
    let url_input = url_input.trim().to_string();

    // Validate URL
    let url = Url::parse(&url_input)?;

    // Get number of connections
    print!("Enter number of connections (1-32, default 4): ");
    io::stdout().flush().unwrap();
    let mut connections_input = String::new();
    io::stdin().read_line(&mut connections_input)?;
    let num_connections = connections_input.trim().parse().unwrap_or(4);

    if num_connections < 1 || num_connections > 32 {
        return Err(DownloaderError::UserInputError("Number of connections must be between 1 and 32".into()));
    }

    // Determine the default filename from the URL path
    let default_filename = url.path_segments()
        .and_then(|segments| segments.last())
        .and_then(|last_segment| Some(last_segment.to_string()))
        .unwrap_or("downloaded_file".to_string());

    print!("Enter output filename (default: {}): ", default_filename);
    io::stdout().flush().unwrap();
    let mut filename_input = String::new();
    io::stdin().read_line(&mut filename_input)?;
    let output_filename = if filename_input.trim().is_empty() {
        default_filename.clone()
    } else {
        let input_trimmed = filename_input.trim();
        if Path::new(input_trimmed).extension().is_none() {
            format!("{}.{}", input_trimmed, Path::new(&default_filename).extension().unwrap_or_default().to_str().unwrap_or("dat"))
        } else {
            input_trimmed.to_string()
        }
    };

    println!("\nInitializing download...");

    let config = DownloadConfig::new(
        url_input,
        output_filename.clone(),
        num_connections,
    );

    let hostname = url.host_str()
        .ok_or_else(|| DownloaderError::UrlParseError(url::ParseError::EmptyHost))?;
    let path = url.path();

    println!("{}", hostname);
    println!("{}", path);

    println!("Resolving hostname...");
    let ip = dns::get_request_ip(hostname)?;

    println!("Establishing connection...");
    let mut stream = connection::establish_tls_connection(hostname, ip)?;

    println!("Checking file details...");
    let (supports_range, total_size) = http::send_head_request(&mut stream, hostname, path)?;

    if !supports_range {
        return Err(DownloaderError::ResponseError(
            "Server does not support range requests".into(),
        ));
    }

    println!("\nFile size: {} bytes", total_size);
    println!("Output file: {}", output_filename);
    println!("Number of connections: {}\n", num_connections);
    println!("Starting download...");

    let manager = DownloadManager::new(config, total_size);
    manager.download()?;

    println!("\nDownload completed successfully!");
    println!("File saved as: {}\n", output_filename);

    Ok(())
}
