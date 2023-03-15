pub fn merge_bytes(upper: u8, lower: u8) -> u16 {
    (upper as u16) << 8 | lower as u16
}

pub fn split_bytes(bytes: u16) -> [u8; 2] {
    [((bytes & 0xFF00) >> 8) as u8, (bytes & 0x00FF) as u8]
}

pub fn bytes_half_carry(a: u8, b: u8) -> bool {
    (a & 0x0F) + (b & 0x0F) > 0x0F
}
