mod palette;
mod ppu;

use crate::emu::display::Display;

use self::ppu::Ppu;

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: u16 = VRAM_END - VRAM_BEGIN + 1;
pub const OAM_BEGIN: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: u16 = OAM_END - OAM_BEGIN + 1;
pub const PPU_REGISTERS: [u16; 5] = [0xFF40, 0xFF42, 0xFF43, 0xFF44, 0xFF47];

pub trait Gpu {
    fn write_vram(&mut self, address: u16, byte: u8);
    fn write_oam(&mut self, address: u16, byte: u8);
    fn write_registers(&mut self, address: u16, byte: u8);
    fn read_vram(&self, address: u16) -> u8;
    fn read_oam(&self, address: u16) -> u8;
    fn read_registers(&self, address: u16) -> u8;
    fn update(&mut self, display: &mut Box<dyn Display>);
    fn inc_ly(&mut self);
    fn lcd_is_enabled(&mut self) -> bool;
}

pub fn new_ppu() -> Box<dyn Gpu> {
    Ppu::new()
}
