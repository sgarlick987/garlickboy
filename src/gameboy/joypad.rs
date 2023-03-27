use crate::controller::Controller;

pub const JOYPAD_ADDRESS: u16 = 0xFF00;

#[derive(Debug)]
enum Selected {
    Directions,
    Actions,
    None,
}

#[derive(Debug)]
pub struct Joypad {
    directions: u8,
    actions: u8,
    selected: Selected,
}

const ACTION_BIT: u8 = 1 << 4;
const DIRECTIONS_BIT: u8 = 1 << 5;

impl Joypad {
    pub fn new() -> Self {
        Self {
            directions: 0xFF,
            actions: 0xFF,
            selected: Selected::None,
        }
    }

    pub fn read(&mut self) -> u8 {
        match self.selected {
            Selected::Actions => self.actions,
            Selected::Directions => self.directions,
            Selected::None => 0xFF,
        }
    }

    pub fn select(&mut self, byte: u8) {
        self.selected = match byte {
            ACTION_BIT => Selected::Actions,
            DIRECTIONS_BIT => Selected::Directions,
            _ => Selected::None,
        }
    }

    pub fn update(&mut self, controller: &Controller) {
        (self.directions, self.actions) = controller.joypad();
    }
}
