use garlickboy::bios;
use garlickboy::cpu;
use garlickboy::rom;
use sdl2::pixels::Color;

const VIDEO_SCALE: u32 = 4;
const GB_ROM: &str = "./data/Tetris.gb";
fn main() {
    let rom = rom::load_rom(GB_ROM);
    let bios = bios::load_bios("data/DMG_ROM.bin");
    let mut cpu = cpu::new_cpu();
    cpu.write_bios(bios);
    cpu.write_rom(rom);
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("GarlickBoy", 160 * VIDEO_SCALE, 144 * VIDEO_SCALE)
        .position_centered()
        .allow_highdpi()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();
    canvas
        .set_scale(VIDEO_SCALE as f32, VIDEO_SCALE as f32)
        .unwrap();
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        if !cpu.step() {
            break;
        }
    }
    loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        cpu.render(&mut canvas);
        canvas.present();
    }
}
