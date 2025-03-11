use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::fileinfo::FileInfo;
use crate::emutrait::Emu;

const UAEFSDB_NAME: &str = "_UAEFSDB.___";

pub struct Amiberry {
    target_dir: PathBuf,
    dir_cache: HashMap<OsString, PathBuf>
}

impl Amiberry {
    pub fn new(path: &Path) -> Self {
        Self {
            target_dir: path.to_owned(),
            dir_cache: HashMap::new(),
        }
    }
}

impl Emu for Amiberry {
    fn get_target_dir(&self) -> PathBuf {
        self.target_dir.clone()
    }
    fn get_dir_cache(&mut self) -> &mut HashMap<OsString, PathBuf> {
        &mut self.dir_cache
    }
    fn write_metadata(&mut self, info: &FileInfo) -> std::io::Result<()> {
        if !Self::needs_metadata(info) {
            return Ok(());
        };
        let aname = info.path_components.iter().next_back().unwrap();
        let nname_db = Self::make_string(aname);
        let nname_fs = self.get_host_path(info);

        let mut db = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .truncate(false)
            .open(nname_fs.with_file_name(UAEFSDB_NAME))?;

        let mut buff = [0u8; 600];
        buff[0] = 0x01;
        buff[3..5].copy_from_slice(&info.protection_bits.to_be_bytes());
        buff[5..5 + aname.len()].copy_from_slice(aname);
        buff[262..262 + nname_db.len()].copy_from_slice(nname_db.as_bytes());

        if let Some(comment) = info.comment {
            buff[519..519 + comment.len()].copy_from_slice(comment);
        };

        db.write_all(&buff)?;

        Ok(())
    }
}

#[test]
fn test_flatten() {
    let s1: &[u8] = &[1, 2];
    let s2: &[u8] = &[3, 4, 5];
    let v = vec![s1, s2];
    let res: Vec<&u8> = v.into_iter().flatten().collect();
    assert_eq!(&res, &[&1, &2, &3, &4, &5]);
}
