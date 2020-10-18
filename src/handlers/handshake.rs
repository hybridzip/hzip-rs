use crate::connection::connection::Connection;
use anyhow::Error;
use std::io::{Read, Write};

pub trait Handshaker {
    fn handshake(&mut self, key: String) -> Result<(), anyhow::Error>;
}

impl Handshaker for Connection {
    fn handshake(&mut self, key: String) -> Result<(), Error> {
        if self.stream.is_none() {
            return anyhow!("Stream was not created");
        }

        let mut stream = self.stream.unwrap();
        let mut token = [0 as u8; 8];

        stream.read(&token)?;

        let mut n = key.len() - 1;

        while n > 0 {
            token[n & 7] ^= key[n];
            n -= 1;
        }

        stream.write(&token)?;



        return Ok(())
    }
}