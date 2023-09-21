use super::{
    lcd::Lcd,
    palette::{Palette, Palettes},
    Gpu, OAM_SIZE, PPU_REGISTERS_ADDRESSES, VRAM_SIZE,
};
use crate::{emu::display::Display, gameboy::interrupts::InterruptHandler};

pub(crate) struct Ppu {
    vram: [u8; VRAM_SIZE as usize],
    oam: [u8; OAM_SIZE as usize],
    palettes: Palettes,
    lcd: Lcd,
    scrollx: u8,
    scrolly: u8,
}

impl Ppu {
    pub fn new() -> Box<dyn Gpu> {
        let lcd = Lcd::new();

        Box::new(Self {
            vram: [0; VRAM_SIZE as usize],
            oam: [0; OAM_SIZE as usize],
            palettes: Palettes::new(),
            lcd,
            scrollx: 0,
            scrolly: 0,
        })
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
            let palette = &self.palettes.background;
            let line_pixels = palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                let x = x + i as u8;
                if x < 8 || x >= 168 || *pixel == palette.zero {
                    continue;
                }
                let x = x - 8;
                display.draw_pixel(x, y, *pixel);
            }
            row += 1;
        }
    }

    fn draw_tile_bg(&mut self, x: u8, y: u8, tile_index: u16, display: &mut Box<dyn Display>) {
        let mut row = 0;
        let tile = self.vram[(tile_index + self.lcd.tile_map_area_address()) as usize];
        let start = (tile as u16) * 16;
        let end = start + 16;

        for line in self.vram[(start as usize)..(end as usize)].chunks(2) {
            let mut y = y + row;
            let scrolly_end = self.scrolly.wrapping_add(143);
            if self.scrolly > scrolly_end {
                if y < self.scrolly && y > (scrolly_end) {
                    row += 1;
                    continue;
                }
                if y >= self.scrolly {
                    y = y - self.scrolly;
                } else {
                    y = y + (143 - scrolly_end);
                }
            } else {
                if y < self.scrolly || y > (scrolly_end) {
                    row += 1;
                    continue;
                }
                y = y - self.scrolly;
            }
            let lower = line[0];
            let upper = line[1];
            let palette = &self.palettes.background;
            let line_pixels = palette.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                let x = x + i as u8;
                if x > 159 {
                    continue;
                }
                display.draw_pixel(x, y, *pixel);
            }
            row += 1;
        }
    }
}

impl Gpu for Ppu {
    fn handles(&self, address: u16) -> bool {
        PPU_REGISTERS_ADDRESSES.contains(&address)
    }

    fn write_vram(&mut self, address: u16, byte: u8) {
        self.vram[address as usize] = byte;
    }

    fn read_vram(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    fn write_oam(&mut self, address: u16, byte: u8) {
        self.oam[address as usize] = byte;
    }

    fn read_oam(&self, address: u16) -> u8 {
        self.oam[address as usize]
    }

    fn write_register(&mut self, address: u16, byte: u8) {
        match address {
            0xFF49 => self.palettes.object1 = Palette::from(byte),
            0xFF48 => self.palettes.object0 = Palette::from(byte),
            0xFF47 => self.palettes.background = Palette::from(byte),
            0xFF45 => self.lcd.write_lyc(byte),
            0xFF44 => (),
            0xFF43 => self.scrollx = byte,
            0xFF42 => self.scrolly = byte,
            0xFF41 => self.lcd.write_stat(byte),
            0xFF40 => self.lcd.write_control(byte),
            _ => panic!("unimplemented write gpu register {:x}", address),
        }
    }

    fn read_register(&self, address: u16) -> u8 {
        match address {
            0xFF49 => self.palettes.object1.byte,
            0xFF48 => self.palettes.object0.byte,
            0xFF47 => self.palettes.background.byte,
            0xFF45 => self.lcd.read_lyc(),
            0xFF44 => self.lcd.read_ly(),
            0xFF43 => self.scrollx,
            0xFF42 => self.scrolly,
            0xFF41 => self.lcd.read_stat(),
            0xFF40 => self.lcd.read_control(),
            _ => panic!("unimplemented read gpu register {:x}", address),
        }
    }

    fn update(&mut self, interrupt_handler: &mut InterruptHandler) {
        self.lcd.update(interrupt_handler);
    }

    fn render_display(&mut self, display: &mut Box<dyn Display>) {
        if !self.lcd.is_lcd_enabled() {
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
}
