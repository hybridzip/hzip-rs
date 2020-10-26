use std::io::{Read, Write};

use anyhow::Error;
use byteorder::{ByteOrder, LittleEndian};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::connection::connection::Connection;
use crate::control::api::ApiCtl;
use crate::control::stream::{DecodeCtl, EncodeCtl, ModelCtl, StreamCtl};
use crate::handlers::session::SessionManager;
use crate::utils::common::{
    read_ctl_string, read_stream, write_ctl_string, write_ctl_word, write_stream,
};

#[derive(Debug, Clone, FromPrimitive, PartialEq)]
pub enum Algorithm {
    Undefined = 0x0,
    Victini = 0x1,
}

pub struct StreamConfig {
    pub filename: String,
    pub algorithm: Option<Algorithm>,
    pub model: Option<String>,
}

pub trait Streamable {
    fn write_file<R: Read>(&mut self, reader: R, config: StreamConfig)
        -> Result<(), anyhow::Error>;

    fn read_file<W: Write>(&mut self, writer: W, config: StreamConfig)
        -> Result<(), anyhow::Error>;

    fn write_model<R: Read>(
        &mut self,
        reader: R,
        config: StreamConfig,
    ) -> Result<(), anyhow::Error>;

    fn read_model<W: Write>(
        &mut self,
        writer: W,
        config: StreamConfig,
    ) -> Result<Option<Algorithm>, anyhow::Error>;
}

impl Streamable for Connection {
    fn write_file<R: Read>(
        &mut self,
        mut reader: R,
        config: StreamConfig,
    ) -> Result<(), anyhow::Error> {
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

        write_stream(stream, &len_bytes)?;
        write_stream(stream, &buf)?;

        read_ctl_string(stream)?;

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

        write_ctl_word(stream, DecodeCtl::Piggyback as u8)?;
        write_ctl_word(stream, DecodeCtl::Stream as u8)?;

        let mut blob_count_buf = [0 as u8; 8];
        read_stream(stream, &mut blob_count_buf)?;

        let blob_count = LittleEndian::read_u64(&blob_count_buf);

        for _ in 0..blob_count {
            let mut len_buf = [0 as u8; 8];
            read_stream(stream, &mut len_buf)?;

            let len = LittleEndian::read_u64(&len_buf);

            let mut buf: Vec<u8> = vec![0 as u8; len as usize];
            read_stream(stream, &mut buf)?;

            writer.write(&buf)?;
        }

        Ok(())
    }

    fn write_model<R: Read>(
        &mut self,
        mut reader: R,
        config: StreamConfig,
    ) -> Result<(), anyhow::Error> {
        if config.algorithm.is_none() {
            return Err(anyhow!("Algorithm was not specified"));
        }

        if config.model.is_none() {
            return Err(anyhow!("Model address was not specified"));
        }

        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();

        write_ctl_word(stream, ApiCtl::Stream as u8)?;
        write_ctl_word(stream, StreamCtl::WriteModel as u8)?;

        write_ctl_word(stream, ModelCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, ModelCtl::Address as u8)?;
        write_ctl_string(stream, config.model.unwrap())?;

        write_ctl_word(stream, ModelCtl::Piggyback as u8)?;

        write_ctl_word(stream, ModelCtl::Stream as u8)?;

        write_ctl_word(stream, config.algorithm.unwrap() as u8)?;

        let mut buf: Vec<u8> = vec![];

        let len = reader.read_to_end(&mut buf)?;

        let mut len_buf = [0 as u8; 8];
        LittleEndian::write_u64(&mut len_buf, len as u64)?;

        write_stream(stream, &len_buf)?;
        write_stream(stream, &buf)?;

        Ok(())
    }

    fn read_model<W: Write>(
        &mut self,
        mut writer: W,
        config: StreamConfig,
    ) -> Result<Option<Algorithm>, Error> {
        if config.model.is_none() {
            return Err(anyhow!("Model address was not specified"));
        }

        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();

        write_ctl_word(stream, ApiCtl::Stream as u8)?;
        write_ctl_word(stream, StreamCtl::ReadModel as u8)?;

        write_ctl_word(stream, ModelCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, ModelCtl::Address as u8)?;
        write_ctl_string(stream, config.model.unwrap())?;

        write_ctl_word(stream, ModelCtl::Piggyback as u8)?;

        write_ctl_word(stream, ModelCtl::Stream as u8)?;

        let mut alg = [0 as u8; 1];
        read_stream(stream, &mut alg)?;

        let mut len_buf = [0 as u8; 8];
        read_stream(stream, &mut len_buf)?;

        let len = LittleEndian::read_u64(&mut len_buf);

        let mut buf = vec![0 as u8; len as usize];
        read_stream(stream, &mut buf)?;

        Ok(FromPrimitive::from_u8(alg[0]))
    }
}
