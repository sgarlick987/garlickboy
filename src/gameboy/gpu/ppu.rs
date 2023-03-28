use crate::emu::display::Display;

use super::{
    palette::{Palette, DEFAULT_PALETTE},
    Gpu, OAM_SIZE, VRAM_SIZE,
};

pub(crate) struct Ppu {
    vram: [u8; VRAM_SIZE as usize],
    oam: [u8; OAM_SIZE as usize],
    palette: Palette,
    pub scrollx: u8,
    pub scrolly: u8,
    pub ly: u8,
    pub lcd_enabled: bool,
}

impl Ppu {
    pub fn new() -> Box<dyn Gpu> {
        Box::new(Self {
            vram: [0; VRAM_SIZE as usize],
            oam: [0; OAM_SIZE as usize],
            palette: DEFAULT_PALETTE,
            scrollx: 0,
            scrolly: 0,
            ly: 0,
            lcd_enabled: false,
        })
    }

    fn set_palette(&mut self, palette: u8) {
        self.palette = Palette::from(palette);
    }

    fn draw_tile_sprite(&mut self, x: u8, y: u8, tile_index: u16, display: &mut Box<dyn Display>) {
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
            let lower = line[0];
            let upper = line[1];
            let line_pixels = self.palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                let x = x + i as u8;
                if x < 8 || x >= 168 || *pixel == self.palette.zero {
                    continue;
                }
                let x = x - 8;
                if *pixel != self.palette.zero {
                    display.draw_pixel(x, y, *pixel);
                }
            }
            row += 1;
        }
    }

    fn draw_tile_bg(&mut self, x: u8, y: u8, tile_index: u16, display: &mut Box<dyn Display>) {
        let mut row = 0;
        let tile = self.vram[(tile_index + 0x1800) as usize];
        let start = (tile as u16) * 16;
        let end = start + 16;

        for line in self.vram[(start as usize)..(end as usize)].chunks(2) {
            let y = y + row;
            let y_start = self.scrolly as u16;
            let mut y_end = y_start + 144;
            if y_end > 255 {
                y_end -= 255;
            }
            if (y_end > y_start && (y as u16 >= y_end || (y as u16) < y_start))
                || (y_end < y_start && (y as u16 + 1 > y_end) && (y as u16 + 1 < y_start))
            {
                continue;
            }

            let lower = line[0];
            let upper = line[1];
            let line_pixels = self.palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                let x = x + i as u8;
                // let x_start = self.scrollx;
                // let x_end = x_start.wrapping_add(160);
                // if (x_end > x_start && (x >= x_end as u32 || x < x_start as u32))
                //     || (x_end < x_start && (x > x_end as u32) && (x <= x_start as u32))
                // {
                //     continue;
                // }
                if x > 159 {
                    continue;
                }
                display.draw_pixel(x, y - self.scrolly, *pixel);
            }
            row += 1;
        }
    }
}

impl Gpu for Ppu {
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
            0xFF47 => self.set_palette(byte),
            0xFF43 => self.scrollx = byte,
            0xFF42 => self.scrolly = byte,
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

    fn update(&mut self, display: &mut Box<dyn Display>) {
        if !self.lcd_enabled {
            display.off();
            return;
        }

        for tile_index in 0u16..1024 {
            let col = (tile_index) % 32;
            let row = (tile_index) / 32;
            self.draw_tile_bg(col as u8 * 8, row as u8 * 8, tile_index, display);
        }

        for sprite_index in 0..40 {
            let y = self.oam[4 * sprite_index];
            let x = self.oam[(4 * sprite_index) + 1];
            let tile_index = self.oam[(4 * sprite_index) + 2];
            let attributes = self.oam[(4 * sprite_index) + 3];
            if y == 0 && x == 0 && tile_index == 0 && attributes == 0 {
                continue;
            }
            self.draw_tile_sprite(x, y, tile_index as u16, display);
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
