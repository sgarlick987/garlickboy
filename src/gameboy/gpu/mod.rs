mod lcd;
mod palette;
mod ppu;

use self::ppu::Ppu;
use super::interrupts::InterruptHandler;
use crate::emu::display::Display;

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: u16 = VRAM_END - VRAM_BEGIN + 1;
pub const OAM_BEGIN: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: u16 = OAM_END - OAM_BEGIN + 1;
const PPU_REGISTERS_ADDRESSES: [u16; 9] = [
    0xFF40, 0xFF41, 0xFF42, 0xFF43, 0xFF44, 0xFF45, 0xFF47, 0xFF48, 0xFF49,
];

pub trait Gpu {
    fn write_vram(&mut self, address: u16, byte: u8);
    fn write_oam(&mut self, address: u16, byte: u8);
    fn write_register(&mut self, address: u16, byte: u8);
    fn read_vram(&self, address: u16) -> u8;
    fn read_oam(&self, address: u16) -> u8;
    fn read_register(&self, address: u16) -> u8;
    fn render_display(&mut self, display: &mut Box<dyn Display>);
    fn update(&mut self, interrupt_handler: &mut InterruptHandler);
    fn handles(&self, address: u16) -> bool;
}

pub fn new_ppu() -> Box<dyn Gpu> {
    Ppu::new()
}
