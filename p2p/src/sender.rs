use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpStream;

struct Sender {
    connections: HashMap<SocketAddr, TcpStream>,
}
