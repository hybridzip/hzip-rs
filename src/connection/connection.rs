use std::net::TcpStream;
use crate::utils::parser::parse_url;

pub struct Connection {
    pub(crate) address: String,
    pub(crate) key: String,
    pub(crate) stream: Option<TcpStream>,
    pub(crate) archive: String,
}

impl Connection {
    pub fn new(url: &str) -> Self {
        let (address, key, archive) = parse_url(url).unwrap();

        Self {
            address,
            key,
            stream: None,
            archive,
        }
    }
}