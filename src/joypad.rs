use sdl2::keyboard::KeyboardState;

pub struct Joypad {
    start: bool,
    select: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
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

impl std::convert::From<KeyboardState<'_>> for Joypad {
    fn from(keyboard_state: KeyboardState<'_>) -> Joypad {
        Joypad {
            start: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::V),
            select: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::C),
            up: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up),
            down: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down),
            left: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left),
            right: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right),
            a: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A),
            b: keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S),
        }
    }
}
