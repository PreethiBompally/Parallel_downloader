use crate::error::DownloaderError;
use std::net::{TcpStream, IpAddr, SocketAddr};
 
pub fn establish_tcp_socket(ip_address: IpAddr, port: u16) -> Result<TcpStream, DownloaderError> {
    let socket_addr = SocketAddr::new(ip_address, port);
    TcpStream::connect(socket_addr)
        .map_err(|e| DownloaderError::ConnectionError(e.to_string()))
}