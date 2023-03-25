use sdl2::keyboard::KeyboardState;

#[derive(Debug)]
pub struct Controller {
    start: bool,
    select: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
}

const DOWN_BIT: u8 = 1 << 3;
const UP_BIT: u8 = 1 << 2;
const LEFT_BIT: u8 = 1 << 1;
const RIGHT_BIT: u8 = 1;
const START_BIT: u8 = 1 << 3;
const SELECT_BIT: u8 = 1 << 2;
const B_BIT: u8 = 1 << 1;
const A_BIT: u8 = 1;

impl Controller {
    pub fn joypad(&self) -> (u8, u8) {
        (
            !((if self.down { DOWN_BIT } else { 0 })
                | (if self.up { UP_BIT } else { 0 })
                | (if self.left { LEFT_BIT } else { 0 })
                | (if self.right { RIGHT_BIT } else { 0 })),
            !((if self.start { START_BIT } else { 0 })
                | (if self.select { SELECT_BIT } else { 0 })
                | (if self.b { B_BIT } else { 0 })
                | (if self.a { A_BIT } else { 0 })),
        )
    }

    pub fn new() -> Controller {
        Controller {
            start: false,
            select: false,
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
        }
    }
}

impl std::convert::From<KeyboardState<'_>> for Controller {
    fn from(keyboard_state: KeyboardState<'_>) -> Controller {
        Controller {
            start: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Return),
            select: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Backspace),
            up: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up),
            down: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down),
            left: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left),
            right: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right),
            a: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::X),
            b: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Z),
        }
    }
}
