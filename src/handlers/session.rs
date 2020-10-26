use crate::connection::connection::Connection;
use crate::control::api::ApiCtl;
use crate::handlers::handshake::Handshaker;
use crate::utils::common::{read_ctl_string, write_stream};
use anyhow::Error;
use std::net::TcpStream;

pub trait SessionManager {
    fn health_check(&mut self) -> Result<(), anyhow::Error>;
    fn refresh_session(&mut self) -> Result<(), anyhow::Error>;
}

impl SessionManager for Connection {
    fn health_check(&mut self) -> Result<(), Error> {
        if self.stream.is_none() {
            return Err(anyhow!("Stream was not created"));
        }

        let stream = self.stream.as_mut().unwrap();
        let word = [ApiCtl::HealthCheck as u8; 1];

        write_stream(stream, &word)?;

        read_ctl_string(stream)?;
        Ok(())
    }

    fn refresh_session(&mut self) -> Result<(), anyhow::Error> {
        if self.health_check().is_err() {
            self.stream = Some(TcpStream::connect(self.address.clone())?);
            self.handshake(self.key.clone())?;
        }

        Ok(())
    }
}
