use crate::display::Display;

use super::{
    bios::Bios,
    gpu::{GPU, PPU_REGISTERS, VRAM_BEGIN, VRAM_END},
};

#[cfg_attr(test, mockall::automock)]
pub trait Bus {
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>);
    fn update_display(&mut self, display: &mut Display);
    fn inc_ly(&mut self);
    fn lcd_is_enabled(&mut self) -> bool;
}

pub struct AddressBus {
    bios: Bios,
    memory: [u8; 0x10000],
    pub gpu: Box<dyn GPU>,
}

impl AddressBus {
    pub fn new(gpu: Box<dyn GPU>, bios: Bios) -> AddressBus {
        AddressBus {
            bios,
            memory: [0; 0x10000],
            gpu,
        }
    }
}

impl Bus for AddressBus {
    fn update_display(&mut self, display: &mut Display) {
        self.gpu.update_display(display);
    }

    fn inc_ly(&mut self) {
        self.gpu.inc_ly();
    }

    fn lcd_is_enabled(&mut self) -> bool {
        self.gpu.lcd_is_enabled()
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        let address = address as usize;
        if PPU_REGISTERS.contains(&address) {
            return self.gpu.read_registers(address);
        }
        match address {
            0x0..=0xFF => {
                if self.bios.mapped {
                    self.bios.data[address]
                } else {
                    self.memory[address]
                }
            }
            0xFF00 => 0b00001111,
            0xFFE4 => self.memory[address],
            0xFFE1 => self.memory[address],
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            _ => self.memory[address],
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;
        if PPU_REGISTERS.contains(&address) {
            self.gpu.write_registers(address, byte);
            return;
        }
        match address {
            0xFF50 => self.bios.mapped = false,
            0xFFE1 => self.memory[address] = byte,
            0xFFE4 => self.memory[address] = byte,
            0x0000..=0x7FFF => (), // ignore writes to rom
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte);
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
