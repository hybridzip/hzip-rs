use std::io::{Read, Write};
use crate::connection::connection::Connection;
use crate::handlers::session::SessionManager;
use crate::utils::common::{write_ctl_word, write_ctl_string, read_ctl_word};
use crate::control::api::{ApiCtl, CommonCtl};
use crate::control::stream::{StreamCtl, EncodeCtl, DecodeCtl};
use byteorder::{LittleEndian, ByteOrder};
use num_derive::FromPrimitive;
use anyhow::Error;


#[derive(Debug, Clone, FromPrimitive, PartialEq)]
pub enum Algorithm {
    Undefined = 0x0,
    Victini = 0x1
}

pub struct StreamConfig {
    pub filename: String,
    pub algorithm: Option<Algorithm>,
    pub model: Option<String>,
}

pub trait Streamable {
    fn write_file<R: Read>(&mut self, reader: R, config: StreamConfig) -> Result<(), anyhow::Error>;

    fn read_file<W: Write>(&mut self, writer: W, config: StreamConfig) -> Result<(), anyhow::Error>;
}

impl Streamable for Connection {
    fn write_file<R: Read>(&mut self, mut reader: R, config: StreamConfig) -> Result<(), anyhow::Error> {
        if config.algorithm.is_none() {
            return Err(anyhow!("Algorithm was not specified"));
        }

        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();

        write_ctl_word(stream, ApiCtl::Stream as u8)?;
        write_ctl_word(stream, StreamCtl::Encode as u8)?;

        write_ctl_word(stream, EncodeCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, EncodeCtl::Destination as u8)?;
        write_ctl_string(stream, config.filename.clone())?;

        write_ctl_word(stream, EncodeCtl::Algorithm as u8)?;
        write_ctl_word(stream, config.algorithm.unwrap().clone() as u8)?;

        if config.model.is_some() {
            write_ctl_word(stream, EncodeCtl::Model as u8)?;
            write_ctl_string(stream, config.model.unwrap())?;
        }

        write_ctl_word(stream, EncodeCtl::Stream as u8)?;

        let mut buf: Vec<u8> = vec![];
        let buf_len = reader.read_to_end(&mut buf)?;

        let mut len_bytes = [0 as u8; 8];
        LittleEndian::write_u64(&mut len_bytes, buf_len as u64);

        stream.write(&mut len_bytes)?;
        stream.write(&mut buf)?;

        read_ctl_word(stream)?;

        Ok(())
    }

    fn read_file<W: Write>(&mut self, mut writer: W, config: StreamConfig) -> Result<(), Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();

        write_ctl_word(stream, ApiCtl::Stream as u8)?;
        write_ctl_word(stream, StreamCtl::Decode as u8)?;

        write_ctl_word(stream, DecodeCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, DecodeCtl::Source as u8)?;
        write_ctl_string(stream, config.filename.clone())?;

        write_ctl_word(stream, DecodeCtl::Algorithm as u8)?;
        write_ctl_word(stream, config.algorithm.unwrap().clone() as u8)?;

        write_ctl_word(stream, DecodeCtl::Piggyback as u8)?;
        write_ctl_word(stream, DecodeCtl::Stream as u8)?;

        while read_ctl_word(stream)? == CommonCtl::PiggyBack {
            let mut len_buf = [0 as u8; 8];
            stream.read(&mut len_buf)?;

            let len = LittleEndian::read_u64(&len_buf);

            let mut buf: Vec<u8> = vec![0 as u8; len as usize];
            stream.read(&mut buf)?;

            writer.write(&buf)?;
        }

        Ok(())
    }
}