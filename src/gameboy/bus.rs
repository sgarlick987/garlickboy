use crate::{controller::Controller, display::Display, utils::merge_bytes};

use super::{
    bios::{Bios, BIOS_MAPPED_ADDRESS},
    gpu::{Gpu, OAM_BEGIN, OAM_END, PPU_REGISTERS, VRAM_BEGIN, VRAM_END},
    joypad::{Joypad, JOYPAD_ADDRESS},
};

#[cfg_attr(test, mockall::automock)]
pub trait Bus {
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>);
    fn update_display(&mut self, display: &mut Display);
    fn update_joypad(&mut self, controller: &Controller);
    fn handle_dma(&mut self);
    fn inc_ly(&mut self);
    fn lcd_is_enabled(&mut self) -> bool;
    fn inc_div(&mut self);
}

const DMA_ADDRESS: u16 = 0xFF46;
const DIVIDER_ADDRESS: u16 = 0xFF04;
const BIOS_ADDRESS_START: u16 = 0x00;
const BIOS_ADDRESS_END: u16 = 0xFF;
const OAM_ADDRESS_START: u16 = 0xFE00;
const HRAM_ADDRESS_START: u16 = 0xFF80;
const HRAM_ADDRESS_END: u16 = 0xFFFE;

struct Dma {
    cycle: u16,
    address: u16,
    in_progress: bool,
}

impl Dma {
    fn start(&mut self, address: u8) {
        self.address = merge_bytes(address, 0x00);
        self.cycle = 0;
        self.in_progress = true;
    }

    fn progress(&mut self) {
        self.cycle += 1;
        if self.cycle == 160 {
            self.cycle = 0;
            self.in_progress = false;
        }
    }
}

pub struct AddressBus {
    bios: Bios,
    divider: u8,
    dma: Dma,
    memory: [u8; 0x10000],
    gpu: Box<dyn Gpu>,
    joypad: Joypad,
}

impl AddressBus {
    pub fn new(gpu: Box<dyn Gpu>, bios: Bios, joypad: Joypad) -> AddressBus {
        let dma = Dma {
            cycle: 0,
            address: 0,
            in_progress: false,
        };

        AddressBus {
            bios,
            divider: 0,
            dma,
            memory: [0; 0x10000],
            gpu,
            joypad,
        }
    }
}

impl AddressBus {
    fn dma_copy(&mut self) {}
}

impl Bus for AddressBus {
    fn handle_dma(&mut self) {
        if self.dma.in_progress {
            let source = self.dma.address + self.dma.cycle;
            let destination = OAM_ADDRESS_START + self.dma.cycle;
            let value = self.read_byte(source);
            self.write_byte(destination, value);
            self.dma.progress();
        }
    }

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
        if PPU_REGISTERS.contains(&address) {
            return self.gpu.read_registers(address);
        }
        match address {
            BIOS_ADDRESS_START..=BIOS_ADDRESS_END => {
                if self.bios.mapped {
                    self.bios.data[address as usize]
                } else {
                    self.memory[address as usize]
                }
            }
            DIVIDER_ADDRESS => self.divider,
            BIOS_MAPPED_ADDRESS => panic!("read from bios mapped address"),
            JOYPAD_ADDRESS => self.joypad.read(),
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            OAM_BEGIN..=OAM_END => self.gpu.read_oam(address - OAM_BEGIN),
            _ => self.memory[address as usize],
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address;
        if PPU_REGISTERS.contains(&address) {
            self.gpu.write_registers(address, byte);
            return;
        }
        match address {
            BIOS_MAPPED_ADDRESS => self.bios.mapped = false,
            JOYPAD_ADDRESS => self.joypad.select(byte),
            DIVIDER_ADDRESS => self.divider = 0,
            DMA_ADDRESS => {
                if byte != 0 {
                    self.dma.start(byte);
                }
            }
            0x0000..=0x7FFF => (), // ignore writes to rom
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, byte),
            OAM_BEGIN..=OAM_END => self.gpu.write_oam(address - OAM_BEGIN, byte),
            _ => {
                self.memory[address as usize] = byte;
            }
        }
    }

    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>) {
        let end = address + bytes.len();
        self.memory[address..end].copy_from_slice(bytes.as_slice());
    }
}
