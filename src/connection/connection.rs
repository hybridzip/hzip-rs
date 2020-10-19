use std::net::TcpStream;

pub struct Connection {
    pub(crate) address: String,
    pub(crate) key: String,
    pub(crate) stream: Option<TcpStream>,
    pub(crate) archive: String,
}

impl Connection {
    pub fn new(address: String, key: String, archive: String) -> Self {
        Self {
            address,
            key,
            stream: None,
            archive,
        }
    }
}