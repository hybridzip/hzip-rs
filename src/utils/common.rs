use std::net::TcpStream;
use std::io::Read;
use crate::control::control::CommonCtl;
use byteorder::ByteOrder;
use num_traits::FromPrimitive;

pub fn read_ctl_word(stream: &mut TcpStream) -> Result<CommonCtl, anyhow::Error> {
    let mut word = [0 as u8; 1];

    stream.read(&mut word)?;

    match FromPrimitive::from_u8(word[0]) {
        Some(CommonCtl::Success) => {
            let mut len_buf = [0 as u8; 8];
            stream.read(&mut len_buf)?;

            let len = byteorder::LittleEndian::read_u64(&len_buf);
            let mut msg_buf = vec![0 as u8; len as usize];

            stream.read(&mut msg_buf)?;

            Ok(CommonCtl::Success)
        }
        Some(CommonCtl::PiggyBack) => Ok(CommonCtl::PiggyBack),
        Some(CommonCtl::Error) => {
            let mut len_buf = [0 as u8; 8];
            stream.read(&mut len_buf)?;

            let len = byteorder::LittleEndian::read_u64(&len_buf);
            let mut msg_buf = vec![0 as u8; len as usize];

            stream.read(&mut msg_buf)?;

            let msg: String = std::str::from_utf8(&msg_buf).unwrap().to_string();

            Err(anyhow!(msg))
        }
        None => Err(anyhow!("Invalid control word was received"))
    }
}