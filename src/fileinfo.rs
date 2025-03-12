use delharc::header::ext::{EXT_HEADER_FILENAME, EXT_HEADER_PATH};
use delharc::header::TimestampResult;
use delharc::LhaHeader;
use std::io::{Error, ErrorKind, Result};

// https://web.archive.org/web/20080724142842/http://homepage1.nifty.com/dangan/en/Content/Program/Java/jLHA/Notes/Notes.html
const EXT_HEADER_LV2_COMMENT: u8 = 0x71; // undocumented!
const PATH_SEPARATOR: u8 = 0xff;

const DEFAULT_FLAGS: [(char, u16); 8] = [
    ('h', 1 << 7),
    ('s', 1 << 6),
    ('p', 1 << 5),
    ('a', 1 << 4),
    ('r', 1 << 3),
    ('w', 1 << 2),
    ('e', 1 << 1),
    ('d', 1 << 0),
];

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub path_components: Vec<&'a [u8]>,
    pub comment: Option<&'a [u8]>,
    pub protection_bits: u16,
    pub is_directory: bool,
    pub timestamp: TimestampResult,
}

// old MS-DOS compatible header created with Amiga "lha -H0"
fn parse_level0(header: &LhaHeader) -> Result<FileInfo> {
    let mut split = header.filename.split(|b| *b == 0);
    let path_bytes = split.next().unwrap();
    let comment = split.next();
    let mut path_components = vec![];
    path_bytes
        .split(|b| *b == b'\\')
        .for_each(|comp| path_components.push(comp));
    Ok(FileInfo {
        path_components,
        comment,
        protection_bits: header.msdos_attrs.bits(),
        is_directory: header.filename.last().unwrap() == &0x5c,
        timestamp: header.parse_last_modified(),
    })
}

// the most common (default) header created by Amiga lha 2.15
fn parse_level1(header: &LhaHeader) -> Result<FileInfo> {
    let mut split = header.filename.split(|b| *b == 0);
    let amiga_file_name = split.next().unwrap();
    let is_directory = if amiga_file_name.is_empty() {
        // weired: Amiga lha uses empy file name
        // for an empty directory instead of -lhd-
        // TODO: could still be a file name is extra header
        true
    } else {
        false
    };
    let comment = split.next();
    let mut path_components = vec![];
    if let Some(dir) = header.iter_extra().find(|e| e[0] == EXT_HEADER_PATH) {
        let range = &dir[1..dir.len() - 1];
        let split = range.split(|b| *b == PATH_SEPARATOR);
        split.for_each(|subdir| path_components.push(subdir));
    }
    if !is_directory {
        path_components.push(amiga_file_name);
    };
    Ok(FileInfo {
        path_components,
        comment,
        protection_bits: header.msdos_attrs.bits(),
        is_directory,
        timestamp: header.parse_last_modified(),
    })
}

// header created with Amiga "lha -H2"
fn parse_level2(header: &LhaHeader) -> Result<FileInfo> {
    let mut amiga_file_name: Option<&[u8]> = None;
    if let Some(name) = header.iter_extra().find(|e| e[0] == EXT_HEADER_FILENAME) {
        amiga_file_name = Some(&name[1..]);
    }
    let mut path_components = vec![];
    if let Some(dir) = header.iter_extra().find(|e| e[0] == EXT_HEADER_PATH) {
        let range = &dir[1..dir.len() - 1];
        let split = range.split(|b| *b == PATH_SEPARATOR);
        split.for_each(|subdir| path_components.push(subdir));
    }
    let mut comment: Option<&[u8]> = None;
    if let Some(c) = header.iter_extra().find(|e| e[0] == EXT_HEADER_LV2_COMMENT) {
        comment = Some(&c[1..]);
    }
    if let Some(file) = amiga_file_name {
        path_components.push(file);
    }
    Ok(FileInfo {
        path_components,
        comment,
        protection_bits: header.msdos_attrs.bits(),
        is_directory: amiga_file_name.is_none(),
        timestamp: header.parse_last_modified(),
    })
}
pub fn parse_file_info(header: &LhaHeader) -> Result<FileInfo> {
    match header.level {
        0 => parse_level0(header),
        1 => parse_level1(header),
        2 => parse_level2(header),
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported LHA header level.",
        )),
    }
}

impl<'a> FileInfo<'a> {
    pub fn get_flags(&self) -> String {
        let bits = self.protection_bits ^ 0b00001111;
        DEFAULT_FLAGS
            .iter()
            .map(|flag| if bits & flag.1 == 0 { '-' } else { flag.0 })
            .collect()
    }
    pub fn get_comment(&self) -> String {
        // FIXME: should be same as make_string()
        self.comment.map_or(String::new(), |bytes| {
            bytes.iter().map(|byte| *byte as char).collect()
        })
    }
    pub fn get_timestamp(&self) -> String {
        match self.timestamp {
            TimestampResult::None => "1980-01-01 00:00:00.00".to_string(),
            TimestampResult::Naive(dt) => dt.format("%Y-%m-%d %H:%M:%S.00").to_string(),
            TimestampResult::Utc(dt) => dt.format("%Y-%m-%d %H:%M:%S.00").to_string(),
        }
    }
}

#[test]
fn test_flags() {
    let expected: &[&str; 8] = &[
        //"hsparwed",
        "---arwed", "----rwed", "----rwe-", "----rw-d", "----r---", "----r-ed", "--p-rwed",
        "-s--rwed",
    ];
    let mut lha = delharc::parse_file("tests/res/bits.lha").unwrap();
    for exp in expected.iter() {
        lha.next_file().unwrap();
        let h = lha.header();
        let i = parse_file_info(h).unwrap();
        assert_eq!(exp, &i.get_flags())
    }
}
