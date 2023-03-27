pub mod palette;
use crate::display::Display;

use self::palette::*;
pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: u16 = VRAM_END - VRAM_BEGIN + 1;
pub const OAM_BEGIN: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: u16 = OAM_END - OAM_BEGIN + 1;

pub trait GPU {
    fn write_vram(&mut self, address: u16, byte: u8);
    fn write_oam(&mut self, address: u16, byte: u8);
    fn write_registers(&mut self, address: u16, byte: u8);
    fn read_vram(&self, address: u16) -> u8;
    fn read_oam(&self, address: u16) -> u8;
    fn read_registers(&self, address: u16) -> u8;
    fn update_display(&mut self, display: &mut Display);
    fn inc_ly(&mut self);
    fn lcd_is_enabled(&mut self) -> bool;
}

pub struct PPU {
    vram: [u8; VRAM_SIZE as usize],
    oam: [u8; OAM_SIZE as usize],
    palette: Palette,
    pub scrollx: u8,
    pub scrolly: u8,
    pub ly: u8,
    pub lcd_enabled: bool,
}

pub const PPU_REGISTERS: [u16; 5] = [0xFF40, 0xFF42, 0xFF43, 0xFF44, 0xFF47];

impl PPU {
    fn draw_tile_sprite(&mut self, x: u32, y: u32, tile_index: u16, display: &mut Display) {
        let mut row = 0;
        let start = tile_index * 16;
        let end = start + 16;
        let x = x;
        let y = y;

        for line in self.vram[(start as usize)..(end as usize)].chunks(2) {
            let y = y + row;
            if y < 16 || y >= 160 {
                continue;
            }
            let y = y - 16;
            let y = y + (self.scrolly as u32);
            let lower = line[0];
            let upper = line[1];
            let line_pixels = self.palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                let x = x + (i as u32);
                if x < 8 || x >= 168 || *pixel == self.palette.zero {
                    continue;
                }
                let x = x - 8;
                let x = x + (self.scrollx as u32);
                if *pixel != self.palette.zero {
                    display.draw_pixel(x, y, *pixel);
                }
            }
            row += 1;
        }
    }

    fn draw_tile_bg(&mut self, x: u32, y: u32, tile_index: u16, display: &mut Display) {
        let mut row = 0;
        let tile = self.vram[(tile_index + 0x1800) as usize];
        let start = (tile as u16) * 16;
        let end = start + 16;

        for line in self.vram[(start as usize)..(end as usize)].chunks(2) {
            let lower = line[0];
            let upper = line[1];
            let line_pixels = self.palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                display.draw_pixel(x + (i as u32), y + row, *pixel);
            }
            row += 1;
        }
    }

    fn draw_sprite(&mut self) {}
}

impl GPU for PPU {
    fn write_vram(&mut self, address: u16, byte: u8) {
        self.vram[address as usize] = byte;
    }

    fn write_oam(&mut self, address: u16, byte: u8) {
        if byte != 0 {
            let _a = 1;
        }
        self.oam[address as usize] = byte;
    }

    fn write_registers(&mut self, address: u16, byte: u8) {
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

    fn read_vram(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    fn read_oam(&self, address: u16) -> u8 {
        self.oam[address as usize]
    }

    fn read_registers(&self, address: u16) -> u8 {
        match address {
            // 0xFF47 => u8::from(self.palette),
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

    fn update_display(&mut self, display: &mut Display) {
        if !self.lcd_enabled {
            display.off();
            return;
        }

        for tile_index in 0u16..1024 {
            let col = (tile_index as u32) % 32;
            let row = (tile_index as u32) / 32;
            self.draw_tile_bg(col * 8, row * 8, tile_index, display);
        }

        for sprite_index in 0..40 {
            let y = self.oam[4 * sprite_index];
            let x = self.oam[(4 * sprite_index) + 1];
            let tile_index = self.oam[(4 * sprite_index) + 2];
            let attributes = self.oam[(4 * sprite_index) + 3];
            if y == 0 && x == 0 && tile_index == 0 && attributes == 0 {
                continue;
            }
            self.draw_tile_sprite(x as u32, y as u32, tile_index as u16, display);
        }
    }

    fn inc_ly(&mut self) {
        if !self.lcd_enabled {
            panic!("gpu inc_ly should not be called while lcd is not enable");
        }

        self.ly += 1;
        if self.ly == 154 {
            self.ly = 0;
        }
    }

    fn lcd_is_enabled(&mut self) -> bool {
        self.lcd_enabled
    }
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            vram: [0; VRAM_SIZE as usize],
            oam: [0; OAM_SIZE as usize],
            palette: DEFAULT_PALETTE,
            scrollx: 0,
            scrolly: 0,
            ly: 0,
            lcd_enabled: false,
        }
    }

    fn set_palette(&mut self, palette: u8) {
        self.palette = Palette::from(palette);
    }
}
