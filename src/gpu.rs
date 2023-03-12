use std::cell::RefCell;

use sdl2::{
    pixels::{Color, PixelFormat},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    EventPump,
};

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const VIDEO_SCALE: u32 = 4;

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    screen: Screen,
    palette: Palette,
    pub scrollx: u8,
    pub scrolly: u8,
}

// #[derive(Copy, Clone)]
pub struct Palette {
    zero: Color,
    one: Color,
    two: Color,
    three: Color,
}

pub struct Screen {
    sdl: sdl2::Sdl,
    canvas: Canvas<sdl2::video::Window>,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<Texture<'static>>,
    data: Vec<u32>,
    width: u32,
    height: u32,
}

impl Screen {
    pub fn event_pump(&self) -> EventPump {
        self.sdl.event_pump().expect("failed to get event_pump")
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.data[(y * self.width + x) as usize] = color
            .to_u32(&PixelFormat::try_from(self.texture_creator.default_pixel_format()).unwrap());
    }

    fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }

    pub fn new() -> Screen {
        let sdl = sdl2::init().expect("failed to init sdl2");
        let video = sdl.video().expect("failed to get video subsystem");
        let window = video
            .window("GarlickBoy", 256 * VIDEO_SCALE, 256 * VIDEO_SCALE)
            .position_centered()
            .allow_highdpi()
            .opengl()
            .build()
            .expect("failed to build window");
        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .expect("failed to convert window into canvas");
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_target(texture_creator.default_pixel_format(), 256, 256)
            .unwrap();

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };

        Screen {
            sdl,
            canvas,
            texture_creator,
            texture: RefCell::new(texture),
            data: vec![0; (256 * 256) as usize],
            width: 256,
            height: 256,
        }
    }
}

const DEFAULT_PALETTE: Palette = Palette {
    zero: Color::WHITE,
    one: Color::RGB(221, 221, 221),
    two: Color::RGB(169, 169, 169),
    three: Color::BLACK,
};

impl std::convert::From<u8> for Palette {
    fn from(byte: u8) -> Self {
        Palette {
            zero: Palette::map_color(byte >> 1 & 1, byte & 1),
            one: Palette::map_color(byte >> 3 & 1, byte >> 2 & 1),
            two: Palette::map_color(byte >> 4 & 1, byte >> 5 & 1),
            three: Palette::map_color(byte >> 7 & 1, byte >> 6 & 1),
        }
    }
}

impl Palette {
    fn bytes_to_color(&self, upper: u8, lower: u8) -> Vec<Color> {
        vec![
            self.decode_color((upper >> 7) & 1, (lower >> 7) & 1),
            self.decode_color((upper >> 6) & 1, (lower >> 6) & 1),
            self.decode_color((upper >> 5) & 1, (lower >> 5) & 1),
            self.decode_color((upper >> 4) & 1, (lower >> 4) & 1),
            self.decode_color((upper >> 3) & 1, (lower >> 3) & 1),
            self.decode_color((upper >> 2) & 1, (lower >> 2) & 1),
            self.decode_color((upper >> 1) & 1, (lower >> 1) & 1),
            self.decode_color(upper & 1, lower & 1),
        ]
    }

    fn decode_color(&self, upper: u8, lower: u8) -> Color {
        match upper {
            1 => match lower {
                1 => self.three,
                0 => self.two,
                _ => panic!("invalid color"),
            },
            0 => match lower {
                0 => self.zero,
                1 => self.one,
                _ => panic!("invalid color"),
            },
            _ => panic!("invalid color"),
        }
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
}

impl GPU {
    pub fn new(screen: Screen) -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            screen,
            palette: DEFAULT_PALETTE,
            scrollx: 0,
            scrolly: 0,
        }
    }

    pub fn render(&mut self) {
        for tile_index in 0u16..1024 {
            let col = (tile_index as u32) % 32;
            let row = (tile_index as u32) / 32;
            self.draw_tile(col * 8, row * 8, tile_index);
        }
        let mut texture = self.screen.texture.borrow_mut();
        texture
            .update(None, self.screen.data_raw(), self.screen.width as usize)
            .expect("failed to update screen texture");
        self.screen
            .canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(Color::BLUE);
                texture_canvas
                    .draw_rect(Rect::new(
                        self.scrollx as i32,
                        self.scrolly as i32,
                        160,
                        144,
                    ))
                    .unwrap();
            })
            .unwrap();
        self.screen.canvas.copy(&texture, None, None).unwrap();
        self.screen.canvas.present();
    }

    fn draw_tile(&mut self, x: u32, y: u32, tile_index: u16) {
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
                self.screen.draw_pixel(x + (i as u32), y + row, *pixel);
            }
            row += 1;
        }
    }

    pub fn set_palette(&mut self, palette: u8) {
        self.palette = Palette::from(palette);
    }

    pub fn write_vram(&mut self, address: usize, bytes: Vec<u8>) {
        let end = address + bytes.len();
        self.vram[address..end].copy_from_slice(bytes.as_slice());
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }
}
