pub mod controller;
pub mod display;
pub mod rom;

use std::process;

use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

use crate::gameboy::Gameboy;

use self::{controller::Controller, display::Display, rom::Rom};

const MAX_MCYCLES_PER_FRAME: u32 = 1050000 / 60;
const GB_ROM: &str = "data/Tetris.gb";

pub struct Emu {
    gameboy: Gameboy,
    fps_manager: FPSManager,
    display: Box<dyn Display>,
    controller: Box<dyn Controller>,
    event_pump: EventPump,
}

impl Emu {
    pub fn new() -> Self {
        let (display, event_pump) = display::new_sdl_display();
        let controller = controller::new_keyboard_controller();
        let rom = Rom::new(GB_ROM);
        let mut gameboy = Gameboy::new();
        gameboy.load_rom(&rom);
        let mut fps_manager = FPSManager::new();
        fps_manager
            .set_framerate(60)
            .expect("failed to set fps_manager framerate to 60");

        Self {
            fps_manager,
            display,
            gameboy,
            event_pump,
            controller,
        }
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

    pub fn run(&mut self) {
        self.init_display();

        let mut cycles_used = 0;
        loop {
            for cycle in self.gameboy.cycles() {
                if cycles_used == 0 {
                    self.handle_events();
                    self.input();
                }

                self.gameboy.execute(cycle);
                cycles_used += 1;

                if cycles_used == MAX_MCYCLES_PER_FRAME {
                    self.present();
                    cycles_used = 0;
                }
            }
        }
    }
}
