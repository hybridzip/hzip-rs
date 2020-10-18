use crate::connection::connection::Connection;
use std::net::TcpStream;
use crate::handlers::handshake::Handshaker;

pub trait SessionManager {
    fn refresh_session(&mut self) -> Result<(), anyhow::Error>;
}

impl SessionManager for Connection {
    fn refresh_session(&mut self) -> Result<(), anyhow::Error> {
        self.stream = Some(TcpStream::connect(self.address.clone())?);
        self.handshake(self.key.clone())?;

        Ok(())
    }
}