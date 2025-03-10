use std::path::{Path, PathBuf};
use crate::fileinfo::FileInfo;
use crate::emutrait::Emu;

pub struct NoEmu {
    target_dir: PathBuf
}

impl NoEmu {
    pub fn new(path: &Path) -> Self {
        Self{target_dir: path.to_owned()}
    }
}

impl Emu for NoEmu {
    fn get_host_path(&self, info: &FileInfo) -> PathBuf {
        let mut result = self.target_dir.clone();
        info.path_components.iter().for_each(|comp|
            result.push(Self::make_string(comp)));
        result
    }

    fn write_metadata(&self, _info: &FileInfo) -> std::io::Result<()> {
        Ok(())
    }
}