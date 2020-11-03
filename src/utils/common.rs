use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use byteorder::{ByteOrder, LittleEndian};
use num_traits::FromPrimitive;

use crate::control::api::CommonCtl;
use std::cmp::min;

pub(crate) fn read_status_word(stream: &mut TcpStream) -> Result<(), anyhow::Error> {
    let mut status = [0 as u8; 1];

    stream.read_exact(&mut status)?;

    match FromPrimitive::from_u8(status[0]) {
        Some(CommonCtl::Success) => Ok(()),
        Some(CommonCtl::Error) => {
            let mut len_buf = [0 as u8; 2];
            stream.read_exact(&mut len_buf)?;

            let len = LittleEndian::read_u16(&mut len_buf);
            let mut buf = vec![0 as u8; len as usize];

            stream.read_exact(&mut buf)?;
            let s = std::str::from_utf8(&buf)?.to_string();

            stream.shutdown(Shutdown::Both)?;

            return Err(anyhow!(s));
        }
        _ => return Err(anyhow!("Undefined word")),
    }
}

pub(crate) fn read_stream(stream: &mut TcpStream, buf: &mut [u8]) -> Result<(), anyhow::Error> {
    read_status_word(stream)?;
    stream.read_exact(buf)?;

    Ok(())
}

pub(crate) fn write_stream(stream: &mut TcpStream, buf: &[u8]) -> Result<(), anyhow::Error> {
    read_status_word(stream)?;
    stream.write(buf)?;
    Ok(())
}

pub(crate) fn write_ctl_string(stream: &mut TcpStream, s: String) -> Result<(), anyhow::Error> {
    let mut len = [0 as u8; 2];
    LittleEndian::write_u16(&mut len, s.len() as u16);

    write_stream(stream, &len)?;
    write_stream(stream, s.as_bytes())?;

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
    write_stream(stream, &buf)?;

    Ok(())
}

pub(crate) fn write_buffer_stream<R: Read>(
    stream: &mut TcpStream,
    mut reader: R,
    length: usize,
) -> Result<(), anyhow::Error> {
    // HZ_RECV manual sync sub-protocol
    // Receive the sync ctl-word
    // Send data in variable-sized buffers

    // Buffer size = 1MB (The buffer can be of any size)

    read_status_word(stream)?;

    let mut n = length;

    while n > 0 {
        let size = min(n, 1048576);
        let mut buf: Vec<u8> = vec![0 as u8; size];

        let m = reader.read(&mut buf)?;
        stream.write(&buf[0..m])?;

        n -= size;
    }

    Ok(())
}

pub(crate) fn write_stream_u64(stream: &mut TcpStream, x: u64) -> Result<(), anyhow::Error> {
    let mut buf = [0 as u8; 8];
    LittleEndian::write_u64(&mut buf, x);

    write_stream(stream, &mut buf)
}
