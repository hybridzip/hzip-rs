use std::io::{Read, Write};
use crate::control::algorithm::Algorithm;
use crate::connection::connection::Connection;
use crate::handlers::session::SessionManager;
use crate::utils::common::{write_ctl_word, write_ctl_string};
use crate::control::api::ApiCtl;
use crate::control::stream::{StreamCtl, EncodeCtl};
use crate::control::stream::StreamCtl::Encode;
use byteorder::{LittleEndian, ByteOrder};

pub struct StreamConfig {
    pub filename: String,
    pub algorithm: Algorithm,
    pub model: Option<String>,
}

pub trait Streamable {
    fn write_file<R: Read>(&mut self, mut reader: R, config: StreamConfig) -> Result<(), anyhow::Error>;
}

impl Streamable for Connection {
    fn write_file<R: Read>(&mut self, mut reader: R, config: StreamConfig) -> Result<(), anyhow::Error> {
        self.refresh_session()?;

        let mut stream = self.stream.unwrap();

        write_ctl_word(&mut stream, ApiCtl::Stream as u8)?;
        write_ctl_word(&mut stream, StreamCtl::Encode as u8)?;

        write_ctl_word(&mut stream, EncodeCtl::Archive as u8)?;
        write_ctl_string(&mut stream, self.archive.clone())?;

        write_ctl_word(&mut stream, EncodeCtl::Destination as u8)?;
        write_ctl_string(&mut stream, config.filename.clone())?;

        write_ctl_word(&mut stream, EncodeCtl::Algorithm as u8)?;
        write_ctl_word(&mut stream, config.algorithm.clone() as u8)?;

        if config.model.is_some() {
            write_ctl_word(&mut stream, EncodeCtl::Model as u8)?;
            write_ctl_string(&mut stream, config.model.unwrap())?;
        }

        write_ctl_word(&mut stream, EncodeCtl::Stream as u8)?;

        let mut buf: Vec<u8> = vec![];
        let buf_len = reader.read_to_end(&mut buf)?;

        let mut len_bytes = [0 as u8; 8];
        LittleEndian::write_u64(&mut len_bytes, buf_len as u64);

        stream.write(&mut len_bytes)?;
        stream.write(&mut buf)?;

        Ok(())
    }
}