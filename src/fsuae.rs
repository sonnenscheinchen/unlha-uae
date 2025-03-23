use crate::emutrait::Emu;
use crate::fileinfo::FileInfo;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::write;
use std::path::{Path, PathBuf};

const METADATA_EXTENSION: &str = ".uaem";

pub struct Fsuae {
    target_dir: PathBuf,
    dir_cache: HashMap<OsString, PathBuf>,
}

impl Fsuae {
    pub fn new(target_dir: PathBuf) -> Self {
        Self {
            target_dir,
            dir_cache: HashMap::new(),
        }
    }
}

impl Emu for Fsuae {
    fn get_target_dir(&self) -> &Path {
        &self.target_dir
    }
    fn get_dir_cache(&mut self) -> &mut HashMap<OsString, PathBuf> {
        &mut self.dir_cache
    }
    fn write_metadata(&mut self, info: &FileInfo) -> std::io::Result<()> {
        if !Self::needs_metadata(info) {
            return Ok(());
        };
        let mut md_path = self.get_host_path(info);
        md_path.as_mut_os_string().push(METADATA_EXTENSION);
        let flags = info.get_flags();
        let datetime = info.get_timestamp();
        let comment = info
            .comment
            .map(Self::make_string)
            .unwrap_or_default();
        let line = format!("{flags} {datetime} {comment}");
        write(&md_path, &line)?;
        Ok(())
    }
    fn make_string(slice: &[u8]) -> String {
        let mut result = String::with_capacity(slice.len() * 3); // worst case alloc
        for &b in slice {
            match b {
                b'%' => result.push_str("%25"),
                b'\\' => result.push_str("%5c"),
                b'*' => result.push_str("%2a"),
                b'?' => result.push_str("%3f"),
                b'"' => result.push_str("%22"),
                b'/' => result.push_str("%2f"),
                b'|' => result.push_str("%7c"),
                b'<' => result.push_str("%3c"),
                b'>' => result.push_str("%3e"),
                _ => result.push(b as char),
            }
        }
        result
    }
}
