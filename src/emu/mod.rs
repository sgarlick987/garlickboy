use std::process;

use crate::{
    controller::Controller,
    display::Display,
    gameboy::bios::*,
    gameboy::{bus::AddressBus, joypad::Joypad},
    gameboy::{gpu::Ppu, GameboyCycle},
    rom::*,
};

use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

use crate::gameboy::Gameboy;

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
        let controller = Controller::new();
        let gameboy = Gameboy::new();
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
        self.controller = Controller::from(self.event_pump.keyboard_state());
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

                self.gameboy.dma();
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
