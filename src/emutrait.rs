use crate::fileinfo::FileInfo;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Result;
use std::path::{Path, PathBuf};

// static TCHAR evilchars[NUM_EVILCHARS] = { '%', '\\', '*', '?', '\"', '/', '|', '<', '>' };
// src/osdep/fsdb_host.cpp
const EVIL_BYTES: [u8; 9] = [0x25, 0x5c, 0x2a, 0x3f, 0x22, 0x2f, 0x7c, 0x3c, 0x3e];

pub trait Emu {
    fn get_target_dir(&self) -> &Path;
    fn get_dir_cache(&mut self) -> &mut HashMap<OsString, PathBuf>;
    fn get_host_path(&mut self, info: &FileInfo) -> PathBuf {
        let mut path = PathBuf::new();
        let dir_cache = Self::get_dir_cache(self);
        for comp in info.path_components.iter() {
            path.push(Self::make_string(comp));
            let path_inner = path.as_os_str().to_ascii_lowercase();
            if let Some(cached) = dir_cache.get(&path_inner) {
                path = cached.clone();
            } else {
                dir_cache.insert(path_inner, path.clone());
            }
        }
        Self::get_target_dir(self).join(path)
    }
    fn write_metadata(&mut self, info: &FileInfo) -> Result<()>;
    fn make_string(slice: &[u8]) -> String;
    fn needs_metadata(info: &FileInfo) -> bool {
        info.comment.is_some() || info.protection_bits != 0 || {
            for byte in info.path_components.iter().copied().flatten() {
                if EVIL_BYTES.iter().any(|evil| evil == byte) {
                    return true;
                }
            }
            return false;
        }
    }
}
