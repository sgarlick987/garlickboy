use crate::{address::*, bios::*, cpu::*, gpu::*, rom::*};
use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

const MAX_CYCLES: u32 = 69905;
const GB_ROM: &str = "./data/Tetris.gb";

pub struct Emu {
    fps_manager: FPSManager,
    cpu: CPU,
    rom: Rom,
    bios: Bios,
    event_pump: EventPump,
}

impl Emu {
    pub fn new() -> Emu {
        let rom = load_rom(GB_ROM);
        let bios = load_bios("data/dmg_rom.bin");
        let screen = Screen::new();
        let event_pump = screen.event_pump();

        let mut fps_manager = FPSManager::new();
        fps_manager
            .set_framerate(60)
            .expect("failed to set fps_manager framerate to 60");
        let gpu = GPU::new(screen);
        let bus = Box::new(AddressBus::new(gpu));
        let cpu = CPU::new(bus);

        Emu {
            fps_manager,
            cpu,
            rom,
            bios,
            event_pump,
        }
    }

    pub fn init(&mut self) {
        self.write_bios();
        self.write_rom();
    }

    fn write_bios(&mut self) {
        self.cpu.write_bytes(0x0000, self.bios.data.to_vec());
    }

    fn write_rom(&mut self) {
        self.cpu
            .write_bytes(0x0100, self.rom.data[0x0100..].to_vec());
    }

    pub fn update(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => {}
            }
        }
        let mut cycles_used = 0;
        while cycles_used < MAX_CYCLES {
            let cycles = self.cpu.step();
            cycles_used += cycles as u32;
        }
        self.cpu.render();
        self.fps_manager.delay();
        true
    }
}
