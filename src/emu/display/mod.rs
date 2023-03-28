mod sdl;

use sdl2::{pixels::Color, EventPump};

use self::sdl::SdlDisplay;

pub const VIDEO_SCALE: u32 = 2;

pub trait Display {
    fn present(&mut self);
    fn off(&mut self);
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color);
}

pub fn new_sdl_display() -> (Box<dyn Display>, EventPump) {
    let sdl = sdl2::init().expect("failed to init sdl2");
    let event_pump = sdl.event_pump().expect("failed to get event_pump");
    (SdlDisplay::new(sdl), event_pump)
}
