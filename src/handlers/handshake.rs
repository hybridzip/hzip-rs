use crate::connection::connection::Connection;
use anyhow::Error;
use std::io::Write;
use crate::utils::common::{read_stream, read_ctl_string};

pub(crate) trait Handshaker {
    fn handshake(&mut self, key: String) -> Result<(), anyhow::Error>;
}

impl Handshaker for Connection {
    fn handshake(&mut self, key: String) -> Result<(), Error> {
        if self.stream.is_none() {
            return Err(anyhow!("Stream was not created"));
        }

        let stream = self.stream.as_mut().unwrap();
        let mut token = [0 as u8; 8];

        read_stream(stream, &mut token)?;

        let mut n = (key.len() - 1) as i32;

        while n >= 0 {
            token[(n as usize) & 7] ^= key.bytes().nth(n as usize).unwrap();
            n -= 1;
        }

        stream.write(&mut token)?;

        read_ctl_string(stream)?;

        return Ok(())
    }
}