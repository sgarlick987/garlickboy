use sdl2::{pixels::Color, rect::Rect, render::Canvas};

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub struct GPU {
    pub vram: [u8; VRAM_SIZE],
    pub palette: CurrentPalette,
}

impl GPU {
    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        let mut row = 0;
        let mut col = 0;
        for tile in self.vram.array_chunks::<16>() {
            self.draw_tile(8 * col, 8 * row, tile, canvas);
            col += 1;
            if col == 20 {
                col = 0;
                row += 1;
            }
        }
    }

    fn draw_tile(&self, x: i32, y: i32, tile: &[u8], canvas: &mut Canvas<sdl2::video::Window>) {
        let mut row = 0;
        for line in tile.array_chunks::<2>() {
            let lower = line[0];
            let upper = line[1];
            let line_pixels = self.bytes_to_color(upper, lower);
            for (i, pixel) in line_pixels.iter().enumerate() {
                canvas.set_draw_color(*pixel);
                canvas
                    .fill_rect(Rect::new(x + (i as i32), y + (row), 1, 1))
                    .unwrap();
            }
            row += 1;
        }
    }

    fn bytes_to_color(&self, upper: u8, lower: u8) -> Vec<Color> {
        vec![
            self.decode_color((upper >> 7) & 0b1, (lower >> 7) & 0b1),
            self.decode_color((upper >> 6) & 0b1, (lower >> 6) & 0b1),
            self.decode_color((upper >> 5) & 0b1, (lower >> 5) & 0b1),
            self.decode_color((upper >> 4) & 0b1, (lower >> 4) & 0b1),
            self.decode_color((upper >> 3) & 0b1, (lower >> 3) & 0b1),
            self.decode_color((upper >> 2) & 0b1, (lower >> 2) & 0b1),
            self.decode_color((upper >> 1) & 0b1, (lower >> 1) & 0b1),
            self.decode_color(upper & 0b1, lower & 0b1),
        ]
    }

    fn decode_color(&self, upper: u8, lower: u8) -> Color {
        match upper {
            1 => match lower {
                1 => self.palette.three,
                0 => self.palette.two,
                _ => panic!("invalid color"),
            },
            0 => match lower {
                0 => self.palette.zero,
                1 => self.palette.one,
                _ => panic!("invalid color"),
            },
            _ => panic!("invalid color"),
        }
    }

    pub fn set_palette(&mut self, palette: u8) {
        self.palette = CurrentPalette {
            zero: map_color(palette >> 1 & 1, palette & 1),
            one: map_color(palette >> 3 & 1, palette >> 2 & 1),
            two: map_color(palette >> 4 & 1, palette >> 5 & 1),
            three: map_color(palette >> 7 & 1, palette >> 6 & 1),
        }
    }

    pub fn write_vram(&mut self, address: usize, bytes: Vec<u8>) {
        let end = address + bytes.len();
        self.vram[address..end].copy_from_slice(bytes.as_slice());
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }
}

pub fn new_gpu() -> GPU {
    GPU {
        vram: [0; VRAM_SIZE],
        palette: CurrentPalette {
            zero: Color::WHITE,
            one: Color::RGB(221, 221, 221),
            two: Color::RGB(169, 169, 169),
            three: Color::BLACK,
        },
    }
}

pub struct CurrentPalette {
    zero: Color,
    one: Color,
    two: Color,
    three: Color,
}

fn map_color(upper: u8, lower: u8) -> Color {
    match upper {
        1 => match lower {
            1 => Color::BLACK,
            0 => Color::RGB(169, 169, 169),
            _ => panic!("invalid color"),
        },
        0 => match lower {
            0 => Color::WHITE,
            1 => Color::RGB(221, 221, 221),
            _ => panic!("invalid color"),
        },
        _ => panic!("invalid color"),
    }
}
