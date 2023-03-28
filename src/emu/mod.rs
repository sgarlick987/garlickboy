pub mod controller;
pub mod display;
mod rom;

use std::process;

use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

use crate::gameboy::{Gameboy, GameboyCycle};

use self::{controller::Controller, display::Display, rom::Rom};

const MAX_MCYCLES_PER_FRAME: u32 = 1050000 / 60;
const GB_ROM: &str = "data/Tetris.gb";

struct EventTimers {
    ly: u8,
    divider: u8,
    vblank: u8,
}

pub struct Emu {
    gameboy: Gameboy,
    fps_manager: FPSManager,
    display: Box<dyn Display>,
    controller: Box<dyn Controller>,
    event_pump: EventPump,
    event_timers: EventTimers,
    rom: Rom,
}

impl Emu {
    pub fn new() -> Emu {
        let (display, event_pump) = display::new_sdl_display();
        let controller = controller::new_keyboard_controller();
        let gameboy = Gameboy::new();
        let mut fps_manager = FPSManager::new();
        fps_manager
            .set_framerate(60)
            .expect("failed to set fps_manager framerate to 60");

        let rom = Rom::new(GB_ROM);
        let event_timers = EventTimers {
            ly: 0,
            vblank: 0,
            divider: 0,
        };

        let mut emu = Emu {
            fps_manager,
            display,
            gameboy,
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
        let event_pump = &self.event_pump;
        self.controller.read(event_pump);
        self.gameboy.update_joypad(&self.controller);
    }

    fn present(&mut self) {
        self.gameboy.update_display(&mut self.display);
        self.display.present();
        self.fps_manager.delay();
    }

    fn init_display(&mut self) {
        self.display.off();
        self.display.present();
        self.fps_manager.delay();
    }

    fn update_timers(&mut self) {
        self.event_timers.divider += 1;
        if self.event_timers.divider == 64 {
            self.gameboy.inc_div();
            self.event_timers.divider = 0;
        }
        if self.gameboy.lcd_is_enabled() {
            self.event_timers.ly += 1;
            if self.event_timers.ly == 114 {
                self.event_timers.ly = 0;
                self.gameboy.inc_ly();
                self.event_timers.vblank += 1;
            }
            if self.event_timers.vblank == 145 {
                self.gameboy.flag_vblank();
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
            for cycle in self.cycles() {
                if cycles_used == 0 {
                    self.handle_events();
                    self.input();
                }

                self.gameboy.execute(cycle);
                self.update_timers();
                cycles_used += 1;

                if cycles_used == MAX_MCYCLES_PER_FRAME {
                    self.present();
                    cycles_used = 0;
                }
            }
        }
    }

    fn cycles(&mut self) -> Box<dyn Iterator<Item = GameboyCycle>> {
        let interrupts = self.gameboy.interrupts();
        let has_interrupts = interrupts.len() != 0;

        if has_interrupts {
            interrupts
        } else {
            self.gameboy.prefetch()
        }
    }

    fn write_rom(&mut self) {
        self.gameboy.write_bytes(0, self.rom.data.to_vec());
    }
}
