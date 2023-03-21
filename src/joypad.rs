use std::process;

use sdl2::{
    event::{self, Event},
    keyboard::Keycode,
    EventPump,
};

pub struct Joypad {
    start: bool,
    select: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    event_pump: EventPump,
}

impl Joypad {
    pub fn new(event_pump: EventPump) -> Joypad {
        Joypad {
            start: false,
            select: false,
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            event_pump,
        }
    }

    pub fn read(&mut self) {
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
}
