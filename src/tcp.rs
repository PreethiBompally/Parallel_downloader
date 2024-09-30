use std::net::TcpStream;

pub fn establish_tcp_socket(ip_address: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect((ip_address, port))
}