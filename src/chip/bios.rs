use std::{fs::File, io::Read, str};

// const BIOS_DMG0_MD5: &str = "32fbbd84168d3482956eb3c5051637f5";
pub const BIOS_MAPPED_ADDRESS: usize = 0xFF50;
pub struct Bios {
    pub data: [u8; 0x100],
    pub mapped: bool,
}

pub fn load_bios(filename: &str) -> Bios {
    let bytes = load_bios_bytes(filename);

    Bios {
        data: bytes,
        mapped: true,
    }
}

fn load_bios_bytes(filename: &str) -> [u8; 0x100] {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = [0; 0x100];

    let size = f.metadata().unwrap().len();
    if size != 256 {
        panic!("expected bios size to be 256 but was {}", size);
    }

    f.read(&mut buffer).unwrap();

    // let mut md5 = Md5::new();
    // md5.update(buffer);
    // let hash = hex::encode(md5.finalize());

    // if hash != BIOS_DMG0_MD5 {
    //     panic!("expected md5 to be {} but was {}", BIOS_DMG0_MD5, hash);
    // }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_bios() {
        load_bios("data/dmg_rom.bin");
    }

    #[test]
    #[should_panic(expected = "no file found")]
    fn test_load_bios_not_found() {
        load_bios("aaaaa");
    }

    #[test]
    #[should_panic(expected = "expected bios size to be 256 but was 62")]
    fn test_load_bios_bad_size() {
        load_bios("data/test/bios/bad_size.bin");
    }

    // #[test]
    // #[should_panic(
    //     expected = "expected md5 to be 32fbbd84168d3482956eb3c5051637f5 but was 11902a0e83b63905033ebc806719bc73"
    // )]
    // fn test_load_bios_bad_md5() {
    //     load_bios("data/test/bios/bad_sha.bin");
    // }
}
