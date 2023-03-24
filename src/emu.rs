use std::process;

use crate::{
    bios::*, chip::address::AddressBus, chip::gpu::PPU, display::Display, joypad::Joypad, rom::*,
};

use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

use crate::chip::GameboyChip;

const MAX_MCYCLES_PER_FRAME: u32 = 1050000 / 60;
const GB_ROM: &str = "./data/Tetris.gb";

struct EventTimers {
    ly: u8,
    vblank: u8,
}

pub struct Emu {
    chip: GameboyChip,
    fps_manager: FPSManager,
    display: Display,
    joypad: Joypad,
    event_pump: EventPump,
    event_timers: EventTimers,
    rom: Rom,
}

impl Emu {
    pub fn new() -> Emu {
        let sdl = sdl2::init().expect("failed to init sdl2");
        let event_pump = sdl.event_pump().expect("failed to get event_pump");
        let display = Display::new(sdl);
        let mut fps_manager = FPSManager::new();
        fps_manager
            .set_framerate(60)
            .expect("failed to set fps_manager framerate to 60");

        let rom = load_rom(GB_ROM);
        let bios = load_bios("data/dmg_boot.bin");
        let joypad = Joypad::new();
        let gpu = Box::new(PPU::new());
        let bus = Box::new(AddressBus::new(gpu, bios));
        let chip = GameboyChip::new(bus);
        let event_timers = EventTimers { ly: 0, vblank: 0 };

        let mut emu = Emu {
            fps_manager,
            display,
            chip,
            rom,
            event_pump,
            event_timers,
            joypad,
        };

        emu.write_rom();

        emu
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => process::exit(0),
                _ => {}
            }
        }
    }

    fn input(&mut self) {
        self.joypad = Joypad::from(self.event_pump.keyboard_state());
        //self.cpu.update_joypad(self.joypad);
    }

    fn present(&mut self) {
        self.chip.update_display(&mut self.display);
        self.display.present();
        self.fps_manager.delay();
    }

    fn init_display(&mut self) {
        self.display.off();
        self.display.present();
        self.fps_manager.delay();
    }

    fn update_timers(&mut self) {
        if self.chip.lcd_is_enabled() {
            self.event_timers.ly += 1;
            if self.event_timers.ly == 114 {
                self.event_timers.ly = 0;
                self.chip.inc_ly();
            }
        } else {
            self.event_timers.ly = 0;
            self.event_timers.vblank = 0;
        }
    }

    pub fn run(&mut self) {
        self.init_display();

        let mut cycles_used = 0;
        loop {
            for step in self.chip.fetch() {
                match cycles_used {
                    0 => {
                        self.handle_events();
                        self.input()
                    }
                    MAX_MCYCLES_PER_FRAME => {
                        self.present();
                        cycles_used = 0;
                    }
                    _ => (),
                }

                self.chip.execute(step);
                self.update_timers();
                cycles_used += 1;
            }
        }
    }

    fn write_rom(&mut self) {
        self.chip.write_bytes(0, self.rom.data.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    #[test]
    fn test_run() {}
}
