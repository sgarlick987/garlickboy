pub mod palette;
use crate::display::Display;

use self::palette::*;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub trait GPU {
    fn write_vram(&mut self, address: usize, byte: u8);
    fn write_registers(&mut self, address: usize, byte: u8);
    fn read_vram(&self, address: usize) -> u8;
    fn read_registers(&mut self, address: usize) -> u8;
    fn update_display(&mut self, display: &mut Display);
    fn inc_ly(&mut self);
    fn lcd_is_enabled(&mut self) -> bool;
}

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    palette: Palette,
    pub scrollx: u8,
    pub scrolly: u8,
    pub ly: u8,
    pub lcd_enabled: bool,
}

pub const PPU_REGISTERS: [usize; 5] = [0xFF40, 0xFF42, 0xFF43, 0xFF44, 0xFF47];

impl PPU {
    fn draw_tile(&mut self, x: u32, y: u32, tile_index: u16, display: &mut Display) {
        let mut row = 0;
        let tile = self.vram[(tile_index + 0x1800) as usize];
        let start = (tile as usize) * 16;
        let end = start + 16;

        if tile != 0 {
            row = row;
        }
        for line in self.vram[start..end].chunks(2) {
            let lower = line[0];
            let upper = line[1];
            let line_pixels = self.palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                display.draw_pixel(x + (i as u32), y + row, *pixel);
            }
            row += 1;
        }
    }
}

impl GPU for PPU {
    fn lcd_is_enabled(&mut self) -> bool {
        self.lcd_enabled
    }

    fn update_display(&mut self, display: &mut Display) {
        if !self.lcd_enabled {
            display.off();
            return;
        }
        for tile_index in 0u16..1024 {
            let col = (tile_index as u32) % 32;
            let row = (tile_index as u32) / 32;
            self.draw_tile(col * 8, row * 8, tile_index, display);
        }
    }

    fn read_registers(&mut self, address: usize) -> u8 {
        match address {
            0xFF44 => self.ly,
            0xFF43 => self.scrollx,
            0xFF42 => self.scrolly,
            0xFF40 => {
                let mut lcd = 0;
                if self.lcd_enabled {
                    lcd |= 1 << 7;
                }
                lcd
            }
            _ => panic!("unimplemented read gpu register {:x}", address),
        }
    }

    fn write_registers(&mut self, address: usize, byte: u8) {
        match address {
            0xFF47 => {
                self.set_palette(byte);
            }
            0xFF43 => {
                self.scrollx = byte;
            }
            0xFF42 => {
                self.scrolly = byte;
            }
            0xFF40 => {
                let lcd_on = byte >> 7 == 1;
                if !self.lcd_enabled && lcd_on {
                    self.ly = 0;
                }
                self.lcd_enabled = lcd_on;
            }
            _ => panic!("unimplemented write gpu register {:x}", address),
        }
    }

    fn write_vram(&mut self, address: usize, byte: u8) {
        self.vram[address] = byte;
    }

    fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    fn inc_ly(&mut self) {
        if !self.lcd_enabled {
            panic!("gpu inc_ly should not be called while lcd is not enable");
        }

        if self.ly == 153 {
            self.ly = 0;
        } else {
            self.ly += 1;
        }
    }

    // fn sync(&mut self) -> u8 {
    //     if !self.lcd_on {
    //         return 0;
    //     }

    //     self.cycle += 4;
    //     if self.cycle == 456 {
    //         self.cycle = 0;
    //         self.ly += 1;
    //     }
    //     if self.ly == 154 {
    //         self.ly = 0;
    //     }
    //     return self.ly;
    // }
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            vram: [0; VRAM_SIZE],
            palette: DEFAULT_PALETTE,
            scrollx: 0,
            scrolly: 0,
            ly: 0,
            lcd_enabled: false,
        }
    }

    // fn draw_tile_old(&mut self, x: u32, y: u32, tile_index: u16) {
    //     let mut row = 0;
    //     let tile = self.vram[(tile_index + 0x1800) as usize];
    //     let start = (tile as usize) * 16;
    //     let end = start + 16;

    //     if tile != 0 {
    //         row = row;
    //     }
    //     for line in self.vram[start..end].chunks(2) {
    //         let lower = line[0];
    //         let upper = line[1];
    //         let line_pixels = self.palette.bytes_to_color(upper, lower);
    //         for (i, pixel) in line_pixels.iter().enumerate() {
    //             self.display.draw_pixel(x + (i as u32), y + row, *pixel);
    //         }
    //         row += 1;
    //     }
    // }

    fn set_palette(&mut self, palette: u8) {
        self.palette = Palette::from(palette);
    }
}
