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
    
    let head_request = format!(
        "HEAD {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host_name
    );
    tls_stream.write_all(head_request.as_bytes())?;

    println!("Successfully established a TCP connection to {}", host_name);
    println!("Basic GET request sent to the server.");
}