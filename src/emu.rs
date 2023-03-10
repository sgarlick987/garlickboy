use crate::{address::AddressBus, cpu::CPU, gpu::GPU};

const VIDEO_SCALE: u32 = 4;

fn new_emu() -> Emu<'static> {
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

    let gpu = &mut GPU::new(canvas);
    let bus = &AddressBus::new(gpu);
    let cpu = &CPU::new(bus);
    Emu { cpu, gpu, bus }
}

struct Emu<'a> {
    cpu: &'a CPU<'a>,
    gpu: &'a GPU,
    bus: &'a AddressBus<'a>,
}
