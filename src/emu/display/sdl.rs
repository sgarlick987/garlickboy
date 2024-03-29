use std::cell::RefCell;

use sdl2::{
    pixels::{Color, PixelFormat},
    render::{Canvas, Texture, TextureCreator},
    Sdl,
};

use super::{Display, VIDEO_HEIGHT, VIDEO_SCALE, VIDEO_WIDTH};

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
    }

    fn off(&mut self) {
        self.data.fill(u32::MAX)
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
        let video_width = VIDEO_WIDTH as u32;
        let video_height = VIDEO_HEIGHT as u32;
        let video = sdl.video().expect("failed to get video subsystem");
        let window = video
            .window(
                "GarlickBoy",
                video_width * VIDEO_SCALE,
                video_height * VIDEO_SCALE,
            )
            .position_centered()
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
            .create_texture_target(
                texture_creator.default_pixel_format(),
                video_width,
                video_height,
            )
            .unwrap();

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };

        let mut display = Box::new(Self {
            canvas,
            texture_creator,
            texture: RefCell::new(texture),
            data: vec![0; (video_width * video_height) as usize],
            width: VIDEO_WIDTH,
        });
        display.off();
        display
    }

    fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }
}
