use crate::connection::connection::Connection;
use anyhow::Error;
use crate::handlers::session::SessionManager;
use crate::control::query::QueryCtl;
use crate::control::api::{ApiCtl, CommonCtl};
use std::io::Read;
use crate::utils::common::{write_ctl_string, write_ctl_word, read_ctl_word};

pub trait FileSystem {
    fn check_if_file_exists(&mut self, filename: String) -> Result<bool, anyhow::Error>;
}

impl FileSystem for Connection {
    fn check_if_file_exists(&mut self, filename: String) -> Result<bool, Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::PiggyBack as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::CheckIfFileExists as u8)?;
        write_ctl_string(stream, filename)?;

        if read_ctl_word(stream)? == CommonCtl::PiggyBack {
            let mut found = [0 as u8; 1];
            stream.read(&mut found)?;

            return Ok(found[0] != 0);
        }

        Ok(false)
    }
}