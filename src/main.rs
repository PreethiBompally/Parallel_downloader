mod dns;
mod tcp;

use std::io::{Write};
use clap::Parser;

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

    // Correcting URL parsing
    let url_parts: Vec<&str> = args.url.split("://").collect();
    let domain_with_path = url_parts.last().unwrap_or(&"");
    let domain_parts: Vec<&str> = domain_with_path.split('/').collect();
    let host_name = domain_parts[0];
    let path = if domain_parts.len() > 1 {
        format!("/{}", domain_parts[1..].join("/"))
    } else {
        "/".to_string() // Assume root if no specific path is provided
    };

    let ip_address = dns::get_request_ip(host_name)?;
    let mut tcp_stream = tcp::establish_tcp_socket(&ip_address, 443)?;

    let head_request = format!(
        "HEAD {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host_name
    );
    tcp_stream.write_all(head_request.as_bytes())?;

    println!("Successfully established a TCP connection to {}", host_name);
    println!("Basic GET request sent to the server.");
    Ok(())
}