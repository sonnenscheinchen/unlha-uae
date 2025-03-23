mod amiberry;
mod arguments;
mod emutrait;
mod fileinfo;
mod fsuae;
mod noemu;
use delharc::OsType;
use emutrait::Emu;
use std::fs::{create_dir_all, File};
use std::io::{copy, Result};
use std::path::PathBuf;

fn uncompress(lha_file: PathBuf, mut emu: impl Emu) -> Result<()> {
    let mut lha_reader = delharc::parse_file(&lha_file)?;

    loop {
        let header = lha_reader.header();
        let info = fileinfo::parse_file_info(header)?;
        let path = emu.get_host_path(&info);

        if header.is_directory() || info.is_directory {
            println!(
                "[D] {} {} {}",
                info.get_flags(),
                info.get_comment(),
                path.strip_prefix(emu.get_target_dir()).unwrap().display()
            );
            create_dir_all(&path)?;
        } else if lha_reader.is_decoder_supported() {
            let directory = path.parent().unwrap();
            if !directory.is_dir() {
                create_dir_all(directory)?;
            };
            if header.parse_os_type() == Ok(OsType::Amiga) || header.level == 0 {
                emu.write_metadata(&info)?;
            };
            println!(
                "[F] {} {} {}",
                info.get_flags(),
                info.get_comment(),
                path.strip_prefix(emu.get_target_dir()).unwrap().display()
            );
            let mut writer = File::create(&path)?;
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
    let source = matches.get_one::<PathBuf>("source").unwrap().clone();
    let target = matches.get_one::<PathBuf>("target").unwrap().clone();

    println!(
        "Unpacking from {} to {}",
        source.display(),
        target.display()
    );

    if matches.get_flag("amiberry") {
        uncompress(source, amiberry::Amiberry::new(target))
    } else if matches.get_flag("fsuae") {
        uncompress(source, fsuae::Fsuae::new(target))
    } else {
        uncompress(source, noemu::NoEmu::new(target))
    }
}
