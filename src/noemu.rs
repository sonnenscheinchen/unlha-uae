use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use crate::fileinfo::FileInfo;
use crate::emutrait::Emu;

pub struct NoEmu {
    target_dir: PathBuf,
    dir_cache: HashMap<OsString, PathBuf>
}

impl NoEmu {
    pub fn new(path: &Path) -> Self {
        Self{
            target_dir: path.to_owned(),
            dir_cache: HashMap::new(),
        }
    }
}

impl Emu for NoEmu {
    fn get_target_dir(&self) -> PathBuf {
        self.target_dir.clone()
    }
    fn get_dir_cache(&mut self) -> &mut HashMap<OsString, PathBuf> {
        &mut self.dir_cache
    }
    fn get_host_path(&mut self, info: &FileInfo) -> PathBuf {
        let mut result = self.target_dir.clone();
        info.path_components.iter().for_each(|comp|
            result.push(Self::make_string(comp)));
        result
    }

    fn write_metadata(&mut self, _info: &FileInfo) -> std::io::Result<()> {
        Ok(())
    }
    fn make_string(slice: &[u8]) -> String {
        slice.iter().map(|byte| *byte as char).collect()
    }
}