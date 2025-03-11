mod arguments;
mod emutrait;
mod fileinfo;
mod amiberry;
mod noemu;
use emutrait::Emu;
use std::fs::{create_dir_all, File};
use std::io::{copy, Result};
use std::path::{Path, PathBuf};
use delharc::OsType;

fn uncompress(lha_file: &Path, mut emu: impl Emu) -> Result<()> {
    let mut lha_reader = delharc::parse_file(lha_file)?;

    loop {
        let header = lha_reader.header();
        let info = fileinfo::parse_file_info(header)?;
        let path = emu.get_host_path(&info);

        if header.is_directory() || info.is_directory {
            create_dir_all(path)?;
        } else if lha_reader.is_decoder_supported() {
            let directory = path.parent().unwrap();
            if !directory.is_dir() {
                create_dir_all(directory)?;
            };
            if header.parse_os_type() == Ok(OsType::Amiga) || header.level == 0 {
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

fn main() -> Result<()> {
    let matches = arguments::get_arg_matches();
    let source = matches.get_one::<PathBuf>("source").unwrap();
    let target = matches.get_one::<PathBuf>("target").unwrap();

    println!(
        "Unpacking from {} to {}",
        source.display(),
        target.display()
    );

    if matches.get_flag("amiberry") {
        uncompress(source, amiberry::Amiberry::new(target))
    } else if matches.get_flag("fsuae") {
        unimplemented!()
    } else {
        uncompress(source, noemu::NoEmu::new(target))
    }
}
