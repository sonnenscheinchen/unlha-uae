use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn get_arg_matches() -> ArgMatches {
    Command::new("unlha-uae")
        .about("LHA archive unpacker targeting Amiga emulators")
        .version("0.2.0")
        .arg(
            Arg::new("source")
                .help("The lha file to unpack")
                .index(1)
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .display_order(10),
        )
        .arg(
            Arg::new("target")
                .help("The target directory to extract to (will be created)")
                .index(2)
                .required(true)
                .value_parser(create_dir)
                .display_order(20),
        )
        .arg(
            Arg::new("fsuae")
                .help("Write metadata for FS-UAE")
                .action(ArgAction::SetTrue)
                .short('f')
                .long("fsuae")
                .conflicts_with("amiberry")
                .display_order(100),
        )
        .arg(
            Arg::new("amiberry")
                .help("Write metadata for Amiberry")
                .action(ArgAction::SetTrue)
                .short('a')
                .long("amiberry")
                .conflicts_with("fsuae")
                .display_order(110),
        )
        .get_matches()
}

fn create_dir(s: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(s);
    match std::fs::read_dir(s) {
        Ok(mut entries) => {
            if entries.next().is_none() {
                Ok(path)
            } else {
                Err(Error::from(ErrorKind::DirectoryNotEmpty))
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => std::fs::create_dir_all(&path).map(|_| path),
            _ => Err(err),
        },
    }
}
