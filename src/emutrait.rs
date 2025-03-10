use std::io::Result;
use std::path::PathBuf;
use crate::fileinfo::FileInfo;

// static TCHAR evilchars[NUM_EVILCHARS] = { '%', '\\', '*', '?', '\"', '/', '|', '<', '>' };
// src/osdep/fsdb_host.cpp
const EVIL_BYTES: [u8; 9] = [0x25, 0x5c, 0x2a, 0x3f, 0x22, 0x2f, 0x7c, 0x3c, 0x3e];

pub trait Emu {
    fn get_host_path(&self, info: &FileInfo) -> PathBuf;
    fn write_metadata(&self, info: &FileInfo) -> Result<()>;
    fn make_string(slice: &[u8]) -> String {
        let mut result = String::with_capacity(slice.len() * 3); // worst case alloc
        for &b in slice {
            match b {
                0xAD => result.push('\u{2014}'),        // soft hyphen to em dash
                0x7F => result.push('\u{2592}'),        // DEL to medium shade block
                0xAA | 0xBA => result.push('\u{FFFD}'), // No underlined small superscript "a" and "o"
                0xA4 => result.push('\u{20AC}'),        // Euro symbol for "modern" Amigas :-)
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