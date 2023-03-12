use garlickboy::emu;
use std::time::Instant;

fn main() {
    let mut emu = emu::Emu::new();
    emu.init();
    let mut start = Instant::now();
    let mut fps: u32 = 0;

    loop {
        if !emu.update() {
            break;
        }
        fps = fps.wrapping_add(1);
        let duration = start.elapsed();
        if duration.as_millis() >= 1000 {
            println!(" fps: {}", fps);
            fps = 0;
            start = Instant::now();
        }
    }
}
