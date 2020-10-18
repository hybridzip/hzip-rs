use std::net::TcpStream;

pub struct Connection {
    address: String,
    key: String,
    pub(crate) stream: Option<TcpStream>,
}

impl Connection {
    pub fn new(address: String, key: String) -> Self {
        Self {
            address,
            key,
            stream: None
        }
    }
}