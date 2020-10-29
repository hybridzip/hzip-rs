use anyhow::Error;
use byteorder::{ByteOrder, LittleEndian};

use crate::connection::connection::Connection;
use crate::control::api::ApiCtl;
use crate::control::query::QueryCtl;
use crate::handlers::session::SessionManager;
use crate::utils::common::{read_ctl_string, read_stream, write_ctl_string, write_ctl_word};

pub trait FileSystem {
    fn file_exists(&mut self, filename: String) -> Result<bool, anyhow::Error>;
    fn all_files(&mut self) -> Result<Vec<String>, anyhow::Error>;
    fn delete_file(&mut self, filename: String) -> Result<(), anyhow::Error>;
    fn delete_model(&mut self, model: String) -> Result<(), anyhow::Error>;
    fn get_mem_usage(&mut self) -> Result<u64, anyhow::Error>;
}

impl FileSystem for Connection {
    fn file_exists(&mut self, filename: String) -> Result<bool, Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::PiggyBack as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::CheckIfFileExists as u8)?;
        write_ctl_string(stream, filename)?;

        let mut found = [0 as u8; 1];
        read_stream(stream, &mut found)?;

        Ok(found[0] != 0)
    }

    fn all_files(&mut self) -> Result<Vec<String>, Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::PiggyBack as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::GetAllFiles as u8)?;

        let mut files: Vec<String> = vec![];

        let mut file_count_buf = [0 as u8; 8];
        read_stream(stream, &mut file_count_buf)?;

        let count = LittleEndian::read_u64(&file_count_buf);

        for _ in 0..count {
            let filename = read_ctl_string(stream)?;
            files.push(filename);
        }

        Ok(files)
    }

    fn delete_file(&mut self, filename: String) -> Result<(), Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::DeleteFile as u8)?;
        write_ctl_string(stream, filename)?;

        Ok(())
    }



    fn delete_model(&mut self, model: String) -> Result<(), Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::DeleteModel as u8)?;
        write_ctl_string(stream, model)?;

        Ok(())
    }

    fn get_mem_usage(&mut self) -> Result<u64, Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;
        write_ctl_word(stream, QueryCtl::GetMemUsage as u8)?;

        let mut usage_buf = [0 as u8; 8];
        read_stream(stream, &mut usage_buf)?;

        Ok(LittleEndian::read_u64(&usage_buf))
    }
}
