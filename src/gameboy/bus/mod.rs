mod bios;
mod dma;
mod joypad;

use crate::emu::{controller::Controller, display::Display, rom::Rom};

use self::{
    bios::{Bios, BIOS_ADDRESS_END, BIOS_ADDRESS_START},
    dma::Dma,
    joypad::{Joypad, JOYPAD_ADDRESS},
};

use super::gpu::{new_ppu, Gpu, OAM_BEGIN, OAM_END, PPU_REGISTERS, VRAM_BEGIN, VRAM_END};

const DMA_ADDRESS: u16 = 0xFF46;
const DIV_ADDRESS: u16 = 0xFF04;
const OAM_ADDRESS_START: u16 = 0xFE00;
const HRAM_ADDRESS_START: u16 = 0xFF80;
const HRAM_ADDRESS_END: u16 = 0xFFFE;
const BIOS_MAPPED_ADDRESS: u16 = 0xFF50;

pub(crate) trait Bus {
    fn is_lcd_vblank(&self) -> bool;
    fn inc_ly(&mut self);
    fn inc_div(&mut self);
    fn is_lcd_enabled(&mut self) -> bool;
    fn update_display(&mut self, display: &mut Box<dyn Display>);
    fn update_joypad(&mut self, controller: &Box<dyn Controller>);
    fn update_dma(&mut self);
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn load_rom(&mut self, rom: &Rom);
}

pub(crate) fn new_address_bus() -> Box<dyn Bus> {
    AddressBus::new()
}

struct AddressBus {
    bios: Bios,
    div: u8,
    dma: Dma,
    memory: [u8; 0x10000],
    gpu: Box<dyn Gpu>,
    joypad: Joypad,
}

impl AddressBus {
    fn new() -> Box<dyn Bus> {
        let gpu = new_ppu();
        let bios = Bios::new();
        let joypad = Joypad::new();
        let dma = Dma::new();

        Box::new(Self {
            bios,
            div: 0,
            dma,
            memory: [0; 0x10000],
            gpu,
            joypad,
        })
    }
}

impl Bus for AddressBus {
    fn is_lcd_vblank(&self) -> bool {
        self.gpu.is_lcd_vblank()
    }
    fn inc_ly(&mut self) {
        self.gpu.inc_ly();
    }

    fn inc_div(&mut self) {
        self.div = self.div.wrapping_add(1);
    }

    fn is_lcd_enabled(&mut self) -> bool {
        self.gpu.is_lcd_enabled()
    }

    fn update_joypad(&mut self, controller: &Box<dyn Controller>) {
        self.joypad.update(controller);
    }

    fn update_display(&mut self, display: &mut Box<dyn Display>) {
        self.gpu.update(display);
    }

    fn update_dma(&mut self) {
        if self.dma.in_progress {
            let source = self.dma.source();
            let destination = self.dma.destination();
            let value = self.read_byte(source);
            self.write_byte(destination, value);
            self.dma.progress();
        }
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
            DIV_ADDRESS => self.div,
            BIOS_MAPPED_ADDRESS => panic!("read from bios mapped address"),
            DMA_ADDRESS => self.dma.upper,
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
            DIV_ADDRESS => self.div = 0,
            DMA_ADDRESS => self.dma.start(byte),
            0x0000..=0x7FFF => (), // ignore writes to rom
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, byte),
            OAM_BEGIN..=OAM_END => self.gpu.write_oam(address - OAM_BEGIN, byte),
            _ => {
                self.memory[address as usize] = byte;
            }
        }
    }

    fn load_rom(&mut self, rom: &Rom) {
        let bytes = rom.data.as_slice();
        let end = bytes.len();
        self.memory[0..end].copy_from_slice(bytes);
    }
}
