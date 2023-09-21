use std::{fs::File, io::Read, str};

pub const BIOS_ADDRESS_START: u16 = 0x00;
pub const BIOS_ADDRESS_END: u16 = 0xFF;
pub const BIOS_MAPPED_ADDRESS: u16 = 0xFF50;

pub struct Bios {
    pub data: [u8; 0x100],
    pub mapped: bool,
}

impl Bios {
    pub fn new() -> Self {
        let mut bios = Self {
            data: [0; 0x100],
            mapped: true,
        };

        bios.load("data/dmg_boot.bin");
        bios
    }

    pub fn load(&mut self, filename: &str) {
        let mut f = File::open(filename).expect("no file found");

        let size = f.metadata().unwrap().len();
        if size != 256 {
            panic!("expected bios size to be 256 but was {}", size);
        }

        f.read(&mut self.data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_bios() {
        let mut bios = Bios::new();
        bios.load("data/dmg_rom.bin");
    }

    #[test]
    #[should_panic(expected = "no file found")]
    fn test_load_bios_not_found() {
        let mut bios = Bios::new();
        bios.load("aaaaa");
    }

    #[test]
    #[should_panic(expected = "expected bios size to be 256 but was 62")]
    fn test_load_bios_bad_size() {
        let mut bios = Bios::new();
        bios.load("data/test/bios/bad_size.bin");
    }
}
