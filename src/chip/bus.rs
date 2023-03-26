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
    fn inc_div(&mut self);
}

const DMA_ADDRESS: usize = 0xFF46;
const DIVIDER_ADDRESS: usize = 0xFF04;
const BIOS_ADDRESS_START: usize = 0x00;
const BIOS_ADDRESS_END: usize = 0xFF;

pub struct AddressBus {
    bios: Bios,
    divider: u8,
    memory: [u8; 0x10000],
    gpu: Box<dyn GPU>,
    joypad: Joypad,
}

impl AddressBus {
    pub fn new(gpu: Box<dyn GPU>, bios: Bios, joypad: Joypad) -> AddressBus {
        AddressBus {
            bios,
            divider: 0,
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

    fn inc_div(&mut self) {
        self.divider = self.divider.wrapping_add(1);
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
            BIOS_ADDRESS_START..=BIOS_ADDRESS_END => {
                if self.bios.mapped {
                    self.bios.data[address]
                } else {
                    self.memory[address]
                }
            }
            DIVIDER_ADDRESS => self.divider,
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
            DIVIDER_ADDRESS => {
                self.divider = 0;
            }
            // DMA_ADDRESS => {
            //     self.dma = true;
            // }
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
