use crate::{controller::Controller, display::Display};

use super::{
    bios::{Bios, BIOS_MAPPED_ADDRESS},
    gpu::{GPU, PPU_REGISTERS, VRAM_BEGIN, VRAM_END},
    joypad::{Joypad, JOYPAD_ADDRESS},
};

#[cfg_attr(test, mockall::automock)]
pub trait Bus {
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>);
    fn update_display(&mut self, display: &mut Display);
    fn update_joypad(&mut self, controller: &Controller);
    fn inc_ly(&mut self);
    fn lcd_is_enabled(&mut self) -> bool;
}

pub struct AddressBus {
    bios: Bios,
    memory: [u8; 0x10000],
    pub gpu: Box<dyn GPU>,
    pub joypad: Joypad,
}

impl AddressBus {
    pub fn new(gpu: Box<dyn GPU>, bios: Bios, joypad: Joypad) -> AddressBus {
        AddressBus {
            bios,
            memory: [0; 0x10000],
            gpu,
            joypad,
        }
    }
}

impl Bus for AddressBus {
    fn update_joypad(&mut self, controller: &Controller) {
        self.joypad.update(controller);
    }

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
            BIOS_MAPPED_ADDRESS => panic!("read from bios mapped address"),
            JOYPAD_ADDRESS => {
                let byte = self.joypad.read();
                return byte;
            }
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
            BIOS_MAPPED_ADDRESS => self.bios.mapped = false,
            JOYPAD_ADDRESS => self.joypad.select(byte),
            0xFF80 => {
                self.memory[address] = byte;
            }
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
