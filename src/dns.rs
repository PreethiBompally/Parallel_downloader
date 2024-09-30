use std::net::ToSocketAddrs;

pub fn get_request_ip(host_name: &str) -> Result<String, std::io::Error> {
    let socket_addr = (host_name, 80).to_socket_addrs()?;
    let ip = socket_addr
        .filter(|addr| addr.is_ipv4())
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No IPv4 address found"))?;
    Ok(ip.ip().to_string())
}