use crate::gpu::*;

pub struct AddressBus {
    memory: [u8; 0xFFFF],
    pub gpu: GPU,
}

impl AddressBus {
    pub fn new(gpu: GPU) -> AddressBus {
        AddressBus {
            memory: [0; 0xFFFF],
            gpu,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0xFF44 => 0x90,
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            _ => self.memory[address],
        }
    }

    pub fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        let address = address as usize;
        match address {
            0x9800..=0x9BFF => {
                self.gpu.write_vram(address - VRAM_BEGIN, bytes);
            }
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, bytes);
            }
            0xFF47 => {
                if bytes.len() != 1 {
                    panic!(
                        "only one byte should be supplied when writing to palette address 0xFF47"
                    );
                }
                self.gpu.set_palette(bytes[0]);
            }
            0xFF42 => {
                if bytes.len() != 1 {
                    panic!(
                        "only one byte should be supplied when writing to scrollx address 0xFF42"
                    );
                }
                self.gpu.scrolly = bytes[0];
            }
            0xFF43 => {
                if bytes.len() != 1 {
                    panic!(
                        "only one byte should be supplied when writing to scrolly address 0xFF43"
                    );
                }
                self.gpu.scrollx = bytes[0];
            }
            _ => {
                let end = address + bytes.len();
                self.memory[address..end].copy_from_slice(bytes.as_slice());
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.write_bytes(address, [byte].to_vec());
    }
}
