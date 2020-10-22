use std::net::TcpStream;
use std::io::{Read, Write};
use crate::control::api::CommonCtl;
use byteorder::{ByteOrder, LittleEndian};
use num_traits::FromPrimitive;

pub(crate) fn read_stream(stream: &mut TcpStream, buf: &mut [u8]) -> Result<(), anyhow::Error> {
    let mut status = [0 as u8; 1];

    stream.read(&mut status)?;

    match FromPrimitive::from_u8(status[0]) {
        Some(CommonCtl::Success) => {
            stream.read(buf)?;
        }
        Some(CommonCtl::Error) => {
            let mut len_buf = [0 as u8; 2];
            stream.read(&mut len_buf)?;

            let len = LittleEndian::read_u16(&mut len_buf);
            let mut buf = vec![0 as u8; len as usize];

            stream.read(&mut buf)?;
            let s = std::str::from_utf8(&buf)?.to_string();

            return Err(anyhow!(s))
        }
        _ => {}
    }

    Ok(())
}

pub(crate) fn write_ctl_string(stream: &mut TcpStream, s: String) -> Result<(), anyhow::Error> {
    let mut len = [0 as u8; 2];
    LittleEndian::write_u16(&mut len, s.len() as u16);

    stream.write(&len)?;
    stream.write(s.as_bytes())?;

    Ok(())
}

pub(crate) fn read_ctl_string(stream: &mut TcpStream) -> Result<String, anyhow::Error> {
    let mut len_buf = [0 as u8; 2];
    read_stream(stream, &mut len_buf)?;

    let len = LittleEndian::read_u16(&mut len_buf);
    let mut buf = vec![0 as u8; len as usize];

    read_stream(stream, &mut buf)?;
    let s = std::str::from_utf8(&buf)?.to_string();

    Ok(s)
}

pub(crate) fn write_ctl_word(stream: &mut TcpStream, word: u8) -> Result<(), anyhow::Error> {
    let buf = [word as u8; 1];
    stream.write(&buf)?;

    Ok(())
}