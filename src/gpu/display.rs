use std::cell::RefCell;

use sdl2::{
    pixels::{Color, PixelFormat},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    EventPump,
};

pub const VIDEO_SCALE: u32 = 4;

pub struct Display {
    sdl: sdl2::Sdl,
    canvas: Canvas<sdl2::video::Window>,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<Texture<'static>>,
    data: Vec<u32>,
    width: u32,
    // height: u32,
}

impl Display {
    pub fn present(&mut self, x: i32, y: i32) {
        let mut texture = self.texture.borrow_mut();
        texture
            .update(None, self.data_raw(), (self.width * 4) as usize)
            .expect("failed to update screen texture");
        self.canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(Color::BLUE);
                texture_canvas.draw_rect(Rect::new(x, y, 160, 144)).unwrap();
            })
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn present_off(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn event_pump(&self) -> EventPump {
        self.sdl.event_pump().expect("failed to get event_pump")
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.data[(y * self.width + x) as usize] = color
            .to_u32(&PixelFormat::try_from(self.texture_creator.default_pixel_format()).unwrap());
    }

    fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }

    pub fn new() -> Display {
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

        Display {
            sdl,
            canvas,
            texture_creator,
            texture: RefCell::new(texture),
            data: vec![0; (256 * 256) as usize],
            width: 256,
            // height: 256,
        }
    }
}
