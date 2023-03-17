use crate::gpu::*;

#[cfg_attr(test, mockall::automock)]
pub trait Bus {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>);
    fn render(&mut self);
    fn sync(&mut self);
}

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
}

impl Bus for AddressBus {
    fn sync(&mut self) {
        self.gpu.sync();
    }

    fn render(&mut self) {
        self.gpu.render();
    }

    fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0xFF44 => self.gpu.ly,
            0xFF42 => self.gpu.scrolly,
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            _ => self.memory[address],
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;
        match address {
            0x9800..=0x9BFF => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte);
            }
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte);
            }
            0xFF47 => {
                self.gpu.set_palette(byte);
            }
            0xFF42 => {
                self.gpu.scrolly = byte;
            }
            0xFF43 => {
                self.gpu.scrollx = byte;
            }
            0xFF40 => {
                let lcd_on = byte >> 7 == 1;
                if !self.gpu.lcd_on && lcd_on {
                    self.gpu.ly = 0;
                }
                self.gpu.lcd_on = lcd_on;
            }
            _ => {
                self.memory[address] = byte;
            }
        }
    }

    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>) {
        let end = address + bytes.len();
        self.memory[address..end].copy_from_slice(bytes.as_slice());
    }
}
