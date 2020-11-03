use anyhow::Error;
use byteorder::{ByteOrder, LittleEndian};

use crate::connection::connection::Connection;
use crate::control::api::ApiCtl;
use crate::control::query::QueryCtl;
use crate::handlers::session::SessionManager;
use crate::utils::common::{read_ctl_string, read_stream, write_ctl_string, write_ctl_word};

#[derive(Debug)]
pub struct FileSystemEntry {
    pub entry: String,
    pub is_file: bool,
}

pub trait Queryable {
    fn file_exists(&mut self, filename: String) -> Result<bool, anyhow::Error>;
    fn list_fs(&mut self, path: String) -> Result<Vec<FileSystemEntry>, anyhow::Error>;
    fn delete_file(&mut self, filename: String) -> Result<(), anyhow::Error>;
    fn delete_model(&mut self, model: String) -> Result<(), anyhow::Error>;
    fn get_mem_usage(&mut self) -> Result<u64, anyhow::Error>;
}

impl Queryable for Connection {
    fn file_exists(&mut self, filename: String) -> Result<bool, Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::PiggyBack as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::Target as u8)?;
        write_ctl_string(stream, filename)?;

        write_ctl_word(stream, QueryCtl::CheckIfFileExists as u8)?;

        let mut found = [0 as u8; 1];
        read_stream(stream, &mut found)?;

        Ok(found[0] != 0)
    }

    fn list_fs(&mut self, path: String) -> Result<Vec<FileSystemEntry>, Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::PiggyBack as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::Target as u8)?;
        write_ctl_string(stream, path)?;

        write_ctl_word(stream, QueryCtl::ListFileSystem as u8)?;

        let mut files: Vec<FileSystemEntry> = vec![];

        let mut file_count_buf = [0 as u8; 8];
        read_stream(stream, &mut file_count_buf)?;

        let count = LittleEndian::read_u64(&file_count_buf);

        for _ in 0..count {
            let filename = read_ctl_string(stream)?;
            let mut alg = [0 as u8; 1];

            read_stream(stream, &mut alg)?;

            files.push(FileSystemEntry {
                entry: filename,
                is_file: alg[0] == 1,
            });
        }

        Ok(files)
    }

    fn delete_file(&mut self, filename: String) -> Result<(), Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::Target as u8)?;
        write_ctl_string(stream, filename)?;

        write_ctl_word(stream, QueryCtl::DeleteFile as u8)?;

        Ok(())
    }

    fn delete_model(&mut self, model: String) -> Result<(), Error> {
        self.refresh_session()?;

        let stream = self.stream.as_mut().unwrap();
        write_ctl_word(stream, ApiCtl::Query as u8)?;

        write_ctl_word(stream, QueryCtl::Archive as u8)?;
        write_ctl_string(stream, self.archive.clone())?;

        write_ctl_word(stream, QueryCtl::Target as u8)?;
        write_ctl_string(stream, model)?;

        write_ctl_word(stream, QueryCtl::DeleteModel as u8)?;

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
