use std::process;

use crate::{
    chip::bios::*,
    chip::gpu::PPU,
    chip::{bus::AddressBus, joypad::Joypad},
    controller::Controller,
    display::Display,
    rom::*,
};

use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

use crate::chip::GameboyChip;

const MAX_MCYCLES_PER_FRAME: u32 = 1050000 / 60;
const GB_ROM: &str = "data/Tetris.gb";

struct EventTimers {
    ly: u8,
    divider: u8,
    vblank: u8,
}

pub struct Emu {
    chip: GameboyChip,
    fps_manager: FPSManager,
    display: Display,
    controller: Controller,
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
        let mut bios = Bios::new("data/dmg_boot.bin");
        bios.load();
        let controller = Controller::new();
        let joypad = Joypad::new();
        let gpu = Box::new(PPU::new());
        let bus = Box::new(AddressBus::new(gpu, bios, joypad));
        let chip = GameboyChip::new(bus);
        let event_timers = EventTimers {
            ly: 0,
            vblank: 0,
            divider: 0,
        };

        let mut emu = Emu {
            fps_manager,
            display,
            chip,
            rom,
            event_pump,
            event_timers,
            controller,
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
        self.controller = Controller::from(self.event_pump.keyboard_state());
        self.chip.update_joypad(&self.controller);
    }

    fn present(&mut self) {
        self.chip.update_display(&mut self.display);
        self.display.present();
        // self.fps_manager.delay();
    }

    fn init_display(&mut self) {
        self.display.off();
        self.display.present();
        self.fps_manager.delay();
    }

    fn update_timers(&mut self) {
        self.event_timers.divider += 1;
        if self.event_timers.divider == 64 {
            self.chip.inc_div();
            self.event_timers.divider = 0;
        }
        if self.chip.lcd_is_enabled() {
            self.event_timers.ly += 1;
            if self.event_timers.ly == 114 {
                self.event_timers.ly = 0;
                self.chip.inc_ly();
                self.event_timers.vblank += 1;
            }
            if self.event_timers.vblank == 145 {
                self.chip.flag_vblank();
            }
            if self.event_timers.vblank == 155 {
                self.event_timers.vblank = 0;
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
            let interrupts = self.chip.interrupts();
            let has_interrupts = interrupts.len() != 0;
            let steps: Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> =
                if has_interrupts {
                    interrupts
                } else {
                    self.chip.fetch()
                };
            for step in steps {
                if cycles_used == 0 {
                    self.handle_events();
                    self.input();
                }

                self.chip.execute(step);
                self.update_timers();
                cycles_used += 1;

                if cycles_used == MAX_MCYCLES_PER_FRAME {
                    self.present();
                    cycles_used = 0;
                }
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
    fn test_run() {
        let byte: u8 = 0x0F;
        println!("{:x}", byte.swap_bytes());
    }
}
