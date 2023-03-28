use std::cell::RefCell;

use sdl2::{
    pixels::{Color, PixelFormat},
    render::{Canvas, Texture, TextureCreator},
    Sdl,
};

use super::{Display, VIDEO_SCALE};

pub(crate) struct SdlDisplay {
    canvas: Canvas<sdl2::video::Window>,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<Texture<'static>>,
    data: Vec<u32>,
    width: u8,
}

impl Display for SdlDisplay {
    fn present(&mut self) {
        let mut texture = self.texture.borrow_mut();
        texture
            .update(None, self.data_raw(), (self.width as u32 * 4) as usize)
            .expect("failed to update screen texture");
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
        self.canvas.clear();
    }

    fn off(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
    }

    fn draw_pixel(&mut self, x: u8, y: u8, color: Color) {
        let y = y as u32;
        let x = x as u32;
        let width = self.width as u32;
        self.data[(y * width + x) as usize] = color
            .to_u32(&PixelFormat::try_from(self.texture_creator.default_pixel_format()).unwrap());
    }
}

impl SdlDisplay {
    pub(crate) fn new(sdl: Sdl) -> Box<dyn Display> {
        let video = sdl.video().expect("failed to get video subsystem");
        let window = video
            .window("GarlickBoy", 160 * VIDEO_SCALE, 144 * VIDEO_SCALE)
            .position_centered()
            .allow_highdpi()
            .opengl()
            .build()
            .expect("failed to build window");
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .expect("failed to convert window into canvas");
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_target(texture_creator.default_pixel_format(), 160, 144)
            .unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };

        Box::new(Self {
            canvas,
            texture_creator,
            texture: RefCell::new(texture),
            data: vec![0; (160 * 144) as usize],
            width: 160,
        })
    }

    fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }
}
