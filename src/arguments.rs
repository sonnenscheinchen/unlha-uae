use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

pub fn get_arg_matches() -> ArgMatches {
    Command::new("unlha-uae")
        .about("LHA archive unpacker targeting Amiga emulators")
        .version("0.1.0")
        .arg(
            Arg::new("source")
                .help("The lha file to unpack")
                //.value_name("IMAGE/DIRECTORY")
                .index(1)
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .display_order(10),
        )
        .arg(
            Arg::new("target")
                .help("The target directory to extract to (will be created)")
                //.value_name("IMAGE/DIRECTORY")
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
                .display_order(100),
        )
        .arg(
            Arg::new("amiberry")
                .help("Write metadata for Amiberry")
                .action(ArgAction::SetTrue)
                .short('a')
                .long("amiberry")
                .display_order(110),
        )
        .get_matches()
}

fn create_dir(s: &str) -> Result<PathBuf, std::io::Error> {
    std::fs::create_dir_all(s).and(Ok(PathBuf::from(s)))
}
