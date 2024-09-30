mod dns;
mod tcp;
mod tls;
mod file_utils;
mod downloader;

use std::io::{self, Read, Write};
use clap::Parser;
use std::thread;
use std::fs::OpenOptions;
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    url: String,

    #[clap(short, long)]
    output: String,

    #[clap(short, long, default_value_t = 4)]
    num_connections: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (host_name, path) = args.url.split_once('/').ok_or("Invalid URL")?;
    let path = format!("/{}", path);

    let ip_address = dns::get_request_ip(host_name)?;
    let tcp_stream = tcp::establish_tcp_socket(&ip_address, 443)?;
    
    // Send HEAD request to get content length
    let head_request = format!(
        "HEAD {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host_name
    );
    tls_stream.write_all(head_request.as_bytes())?;

    let mut response = String::new();
    tls_stream.read_to_string(&mut response)?;

    let content_length: usize = response
        .lines()
        .find(|line| line.to_lowercase().starts_with("content-length:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|len| len.trim().parse().ok())
        .ok_or("Content-Length not found")?;

    let part_length = content_length / args.num_connections;
    let mut handles = vec![];

    for i in 0..args.num_connections {
        let start = i * part_length;
        let end = if i == args.num_connections - 1 {
            content_length - 1
        } else {
            (i + 1) * part_length - 1
        };

        let host_name = host_name.to_string();
        let path = path.clone();
        let output_file_name = args.output.clone();

        handles.push(thread::spawn(move || {
            downloader::download_part(&host_name, &path, start, end, i, &output_file_name)
        }));
    }

    // for handle in handles {
    //     // handle.join().unwrap()?;
    //     match handle.join() {
    //         Ok(result) => result?,
    //         Err(e) => return Err(format!("Thread panicked: {:?}", e).into()),
    //     }
    // }
    for handle in handles {
        if let Err(e) = handle.join() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Thread panicked: {:?}", e),
            )));
        }
    }

    // TODO: Implement file merging logic here
    merge_files(args.output, args.num_connections)?;
    Ok(())
}

fn merge_files(output_file: String, num_parts: usize) -> io::Result<()> {
    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(output_file.clone())?;

    for i in 0..num_parts {
        let part_file_name = format!("{}.part{}", output_file, i);
        let mut part_file = File::open(&part_file_name)?;

        let mut buffer = vec![];
        part_file.read_to_end(&mut buffer)?;
        output.write_all(&buffer)?;

        // Optionally, you can remove the part file after merging
        // std::fs::remove_file(part_file_name)?;
    }

    Ok(())
}