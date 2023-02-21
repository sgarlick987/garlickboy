use std::fs;
use std::io::Read;
use std::fs::File;
use std::str::from_utf8;
use std::num::Wrapping;

#[derive(Debug)]
#[repr(u8)]
enum ColorModes {
    Enhanced = 0x80,
    Required = 0xC0,
    None = 0x00,
}

#[derive(Debug)]
#[repr(u8)]
enum DestinationCode {
    Japan = 0x00,
    OverseasOnly = 0x01
}

#[derive(Debug)]
#[repr(u8)]
enum RomSize {
    K32 = 0x00,
    K64 = 0x01,
    K128 = 0x02,
    K256 = 0x03,
    K512 = 0x04,
    M1 = 0x05,
    M2 = 0x06,
    M4 = 0x07,
    M5 = 0x08,
    M11 = 0x52,
    M12 = 0x53,
    M15 = 0x54
}

#[derive(Debug)]
#[repr(u8)]
enum RamSize {
    NoRam = 0x00,
    Unused = 0x01,
    K8 = 0x02,
    K32 = 0x03,
    K128 = 0x04,
    K64 = 0x05,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RomHeader {
    title: String,
    gbc_mode: ColorModes,
    ram_size: RamSize,
    rom_size: RomSize,
    destination_code: DestinationCode,
    header_checksum: u8,
    global_checksum: u16,
}

#[derive(Debug)]
pub struct Rom {
    pub header: RomHeader,
    pub data: Vec<u8>,
}

pub fn load_rom(path: String) -> Rom {
    let data = load_rom_bytes(path);
    let header = load_rom_header(data.clone());
    let generated_header_checksum = generate_header_checksum(data.clone());
    let generated_global_checksum = generate_global_checksum(data.clone());

    if generated_global_checksum != header.global_checksum || generated_header_checksum != header.header_checksum {
        println!("checksum mismatch");
        println!("generated global checksum: {}", generated_global_checksum);
        println!("loaded global checksum: {}", header.global_checksum);
        println!("generated header checksum: {}", generated_header_checksum);
        println!("loaded header checksum: {}", header.header_checksum);
    }

    println!("{:#?}", header);
    Rom {
        header: header,
        data: data[0x0150..].to_vec(),
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
    println!("{:#X?}", rom_header_raw);
    let title = from_utf8(&rom_header_raw.title).expect("invalid utf-8 sequence").to_string();
    let gbc_mode: ColorModes = unsafe { ::std::mem::transmute(rom_header_raw.cgb_flag) };
    let ram_size: RamSize = unsafe { ::std::mem::transmute(rom_header_raw.ram_size) };
    let rom_size: RomSize = unsafe { ::std::mem::transmute(rom_header_raw.rom_size) };
    let destination_code: DestinationCode = unsafe { ::std::mem::transmute(rom_header_raw.destination_code) };

    RomHeader {
        title: title,
        gbc_mode: gbc_mode,
        ram_size: ram_size,
        rom_size: rom_size,
        destination_code: destination_code,
        header_checksum: rom_header_raw.header_checksum,
        global_checksum:  ((rom_header_raw.global_checksum[0] as u16) << 8) | rom_header_raw.global_checksum[1] as u16,
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

fn load_rom_bytes(filename: String) -> Vec<u8> {
    let mut f = File::open(filename.clone()).expect("no file found");
    let metadata = fs::metadata(filename.clone()).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}