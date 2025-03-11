# About
Using directories as emulated hard drives in *uae is really convenient. Get your Amiga stuff from Aminet, scene.org or pouet and use 7zip or whatever tool the extract. Easy. Well not always. If you follow the Amiga community you find some people saying "Never unpack LHA files on the PeeCee. You will run into issues." Hm, the situation isn't that bad. Most file will unpack just fine with regular tools but not all. `unlha-uae` ~~will~~ tries to unpack all files correctly, ready to use with your favourite Amiga emulator.
Note: Currently FS-UAE is not supported yet.

# How to build
Install `rust` and `cargo` and clone the repo.
```
cd unlha-uae
cargo build --release
```
The binary is `target/release/unlha-uae`. Copy it anywhere you like.

# Example files
- `Embryo_v1.0_2265.lha` from Retroplay's WHDLoad collection. Heavily relies on file-notes. Game will not work without it. unlha-uae writes file-notes/comments to special metadata files that emulators will use.
- `mui38usr.lha` from Aminet. Contains lower- and upper case `mui` and `MUI` directory. Emulator(s) will not combine them. You end up with missing files. unlha-uae will take care of that.
- `mui38usr.lha` again. Contains non-ASCII characters. You really want to see `português`, `français` and `español` on the host- **and** the emulated system, not something like `portuguÃês/`, `fran�ais` or `espa?ol`

# Usage
```
$ unlha-uae --help
LHA archive unpacker targeting Amiga emulators

Usage: unlha-uae [OPTIONS] <source> <target>

Arguments:
  <source>  The lha file to unpack
  <target>  The target directory to extract to (will be created)

Options:
  -f, --fsuae     Write metadata for FS-UAE
  -a, --amiberry  Write metadata for Amiberry
  -h, --help      Print help
  -V, --version   Print version
```
