use sdl2::{keyboard::Scancode, EventPump};

use super::*;

pub(crate) struct Keyboard {
    start: bool,
    select: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start_key: Scancode,
    select_key: Scancode,
    up_key: Scancode,
    down_key: Scancode,
    left_key: Scancode,
    right_key: Scancode,
    a_key: Scancode,
    b_key: Scancode,
}

impl Keyboard {
    pub fn new() -> Box<dyn Controller> {
        Box::new(Self {
            start: false,
            select: false,
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start_key: sdl2::keyboard::Scancode::Return,
            select_key: sdl2::keyboard::Scancode::Backspace,
            up_key: sdl2::keyboard::Scancode::Up,
            down_key: sdl2::keyboard::Scancode::Down,
            left_key: sdl2::keyboard::Scancode::Left,
            right_key: sdl2::keyboard::Scancode::Right,
            a_key: sdl2::keyboard::Scancode::X,
            b_key: sdl2::keyboard::Scancode::Z,
        })
    }
}

impl Controller for Keyboard {
    fn read(&mut self, event_pump: &EventPump) {
        let keyboard_state = event_pump.keyboard_state();
        self.start = keyboard_state.is_scancode_pressed(self.start_key);
        self.select = keyboard_state.is_scancode_pressed(self.select_key);
        self.up = keyboard_state.is_scancode_pressed(self.up_key);
        self.down = keyboard_state.is_scancode_pressed(self.down_key);
        self.left = keyboard_state.is_scancode_pressed(self.left_key);
        self.right = keyboard_state.is_scancode_pressed(self.right_key);
        self.a = keyboard_state.is_scancode_pressed(self.a_key);
        self.b = keyboard_state.is_scancode_pressed(self.b_key);
    }

    fn actions(&self) -> u8 {
        !((if self.start { START_BIT } else { 0 })
            | (if self.select { SELECT_BIT } else { 0 })
            | (if self.b { B_BIT } else { 0 })
            | (if self.a { A_BIT } else { 0 }))
    }

    fn directions(&self) -> u8 {
        !((if self.down { DOWN_BIT } else { 0 })
            | (if self.up { UP_BIT } else { 0 })
            | (if self.left { LEFT_BIT } else { 0 })
            | (if self.right { RIGHT_BIT } else { 0 }))
    }
}
