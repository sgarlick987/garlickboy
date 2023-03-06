use std::fs::File;
use std::io::Read;

pub struct Bios {
    pub data: [u8; 0xFF],
}

pub fn load_bios(filename: &str) -> Bios {
    let bytes = load_bios_bytes(filename);

    Bios { data: bytes }
}

fn load_bios_bytes(filename: &str) -> [u8; 0xFF] {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = [0; 0xFF];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
