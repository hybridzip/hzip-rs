use crate::connection::connection::Connection;
use std::net::TcpStream;
use crate::handlers::handshake::Handshaker;
use anyhow::Error;
use std::io::Write;
use crate::control::api::ApiCtl;
use crate::utils::common::read_ctl_string;

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
        let mut word = [ApiCtl::HealthCheck as u8; 1];

        stream.write(&mut word)?;

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