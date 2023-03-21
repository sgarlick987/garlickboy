use garlickboy::emu::Emu;
use sdl2::pixels::Color;
use std::time::Instant;

fn main() {
    let sdl = sdl2::init().expect("failed to init sdl2");
    let video = sdl.video().expect("failed to get video subsystem");
    let window = video
        .window("GarlickBoy", 256 * 4, 256 * 4)
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
    loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
    }
    // let mut emu = Emu::new();
    // let mut start = Instant::now();
    // let mut fps: u32 = 0;

    // loop {
    //     if !emu.run() {
    //         break;
    //     }
    //     fps = fps.wrapping_add(1);
    //     let duration = start.elapsed();
    //     if duration.as_millis() >= 1000 {
    //         println!(" fps: {}", fps);
    //         fps = 0;
    //         start = Instant::now();
    //     }
    // }
}
