#![allow(dead_code)]
use std::fs;
use std::fs::File;
use std::io::Read;
use std::num::Wrapping;
use std::str::from_utf8;

struct RomHeaderRaw {
    old_licensee_code: u8,
    destination_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: Vec<u8>,
    title: Vec<u8>,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    cgb_flag: u8,
    sgb_flag: u8,
    entrypoint: Vec<u8>,
    logo: Vec<u8>,
}

pub struct RomHeader {
    title: String,
    header_checksum: u8,
    global_checksum: u16,
}

pub struct Rom {
    header: RomHeader,
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Self {
        let data = load_rom_bytes(path);
        let header = load_rom_header(data.clone());
        let generated_header_checksum = generate_header_checksum(data.clone());
        let generated_global_checksum = generate_global_checksum(data.clone());

        if generated_global_checksum != header.global_checksum
            || generated_header_checksum != header.header_checksum
        {
            println!("checksum mismatch");
            println!("generated global checksum: {}", generated_global_checksum);
            println!("loaded global checksum: {}", header.global_checksum);
            println!("generated header checksum: {}", generated_header_checksum);
            println!("loaded header checksum: {}", header.header_checksum);
        }

        Self {
            header: header,
            data: data.to_vec(),
        }
    }
}

fn load_rom_header_raw(data: Vec<u8>) -> RomHeaderRaw {
    RomHeaderRaw {
        title: data[0x134..0x143].to_vec(),
        cgb_flag: data[0x0143],
        sgb_flag: data[0x0146],
        destination_code: data[0x014A],
        old_licensee_code: data[0x014B],
        mask_rom_version_number: data[0x014C],
        header_checksum: data[0x014D],
        global_checksum: data[0x014E..0x0150].to_vec(),
        cart_type: data[0x0147],
        rom_size: data[0x0148],
        ram_size: data[0x0149],
        logo: data[0x0104..0x0134].to_vec(),
        entrypoint: data[0x0100..0x0104].to_vec(),
    }
}

fn load_rom_header(data: Vec<u8>) -> RomHeader {
    let rom_header_raw = load_rom_header_raw(data);
    let title = from_utf8(&rom_header_raw.title)
        .expect("invalid utf-8 sequence")
        .to_string();
    println!("game title: {}", title);

    RomHeader {
        title,
        header_checksum: rom_header_raw.header_checksum,
        global_checksum: ((rom_header_raw.global_checksum[0] as u16) << 8)
            | rom_header_raw.global_checksum[1] as u16,
    }
}

fn generate_header_checksum(data: Vec<u8>) -> u8 {
    let mut checksum = Wrapping(0u8);
    for val in data[0x0134..0x014D].iter() {
        checksum = checksum - Wrapping(*val) - Wrapping(1);
    }

    checksum.0
}

fn generate_global_checksum(data: Vec<u8>) -> u16 {
    let mut checksum = Wrapping(0u16);
    for (index, val) in data.iter().enumerate() {
        if index != 0x014E && index != 0x014F {
            checksum += Wrapping(*val as u16);
        }
    }

    checksum.0
}

fn load_rom_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
