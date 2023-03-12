use garlickboy::emu;
use std::time::Instant;

fn main() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

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
        if duration.as_secs() >= 1 {
            println!(" fps: {}", fps);
            fps = 0;
            start = Instant::now();
        }
    }
}
