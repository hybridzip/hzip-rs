use std::net::TcpStream;

pub struct Connection {
    pub(crate) address: String,
    pub(crate) key: String,
    pub stream: Option<TcpStream>,
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