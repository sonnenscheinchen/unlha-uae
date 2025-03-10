mod amiberry;
mod arguments;
mod emutrait;
mod fileinfo;
mod noemu;
use amiberry::Amiberry;
use delharc::parse_file;
use noemu::NoEmu;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::{Path, PathBuf};
//use anyhow::Result;
use arguments::get_arg_matches;
use emutrait::Emu;
use fileinfo::parse_file_info;

const OS_TYPE_AMIGA: u8 = 0x41;

fn uncompress(lha_file: &Path, emu: impl Emu) -> std::io::Result<()> {
    let mut lha_reader = parse_file(lha_file)?;

    loop {
        let header = lha_reader.header();
        let info = parse_file_info(header)?;
        let path = emu.get_host_path(&info);

        if header.is_directory() || info.is_directory {
            create_dir_all(path)?;
        } else if lha_reader.is_decoder_supported() {
            let directory = path.parent().unwrap();
            if !directory.is_dir() {
                create_dir_all(directory)?;
            };
            if header.os_type == OS_TYPE_AMIGA || header.level == 0 {
                emu.write_metadata(&info)?;
            };
            let mut writer = File::create(path)?;
            copy(&mut lha_reader, &mut writer)?;
            lha_reader.crc_check()?;
        } else {
            eprintln!("WARN: skipping file with unsupported compression method");
        }
        if !lha_reader.next_file()? {
            break;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let matches = get_arg_matches();
    let source = matches.get_one::<PathBuf>("source").unwrap();
    let target = matches.get_one::<PathBuf>("target").unwrap();

    println!(
        "Unpacking from {} to {}",
        source.display(),
        target.display()
    );

    if matches.get_flag("amiberry") {
        uncompress(source, Amiberry::new(target))
    } else if matches.get_flag("fsuae") {
        unimplemented!()
    } else {
        uncompress(source, NoEmu::new(target))
    }
}
