use crate::emu::controller::Controller;

pub(crate) const JOYPAD_ADDRESS: u16 = 0xFF00;

enum Selected {
    Directions,
    Actions,
    None,
}

pub(crate) struct Joypad {
    directions: u8,
    actions: u8,
    selected: Selected,
}

const ACTION_BIT: u8 = 1 << 4;
const DIRECTIONS_BIT: u8 = 1 << 5;

impl Joypad {
    pub(crate) fn new() -> Self {
        Self {
            directions: 0xFF,
            actions: 0xFF,
            selected: Selected::None,
        }
    }

    pub(crate) fn read(&mut self) -> u8 {
        match self.selected {
            Selected::Actions => self.actions,
            Selected::Directions => self.directions,
            Selected::None => 0xFF,
        }
    }

    pub(crate) fn select(&mut self, byte: u8) {
        self.selected = match byte {
            ACTION_BIT => Selected::Actions,
            DIRECTIONS_BIT => Selected::Directions,
            _ => Selected::None,
        }
    }

    pub fn update(&mut self, controller: &Box<dyn Controller>) {
        self.actions = controller.actions();
        self.directions = controller.directions();
    }
}
