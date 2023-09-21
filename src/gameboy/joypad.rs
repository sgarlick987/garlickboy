use crate::emu::controller::Controller;

pub const JOYPAD_ADDRESS: u16 = 0xFF00;
const ACTION_BIT: u8 = 1 << 4;
const DIRECTIONS_BIT: u8 = 1 << 5;

#[derive(PartialEq)]
enum Selected {
    Directions,
    Actions,
    None,
}

pub struct Joypad {
    directions: u8,
    actions: u8,
    selected: Selected,
}

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

    pub fn update(&mut self, controller: &Box<dyn Controller>) {
        self.actions = controller.actions();
        self.directions = controller.directions();
    }
}

#[cfg(test)]
mod tests {
    use crate::emu::controller::MockController;

    use super::*;

    #[test]
    fn test_read_none_selected() {
        let mut joypad = Joypad::new();
        joypad.select(0xFF);
        assert!(joypad.selected == Selected::None);
        assert!(joypad.read() == 0xFF);
    }

    #[test]
    fn test_read_actions_selected() {
        let mut joypad = Joypad::new();
        joypad.actions = 0;
        joypad.select(ACTION_BIT);

        assert!(joypad.selected == Selected::Actions);
        assert!(joypad.read() == 0);
    }

    #[test]
    fn test_read_directions_selected() {
        let mut joypad = Joypad::new();
        joypad.directions = 0;
        joypad.select(DIRECTIONS_BIT);

        assert!(joypad.selected == Selected::Directions);
        assert!(joypad.read() == 0);
    }

    #[test]
    fn test_update() {
        let mut mock = MockController::new();
        mock.expect_actions().once().return_const(0);
        mock.expect_directions().once().return_const(0);
        let controller: Box<dyn Controller> = Box::new(mock);
        let mut joypad = Joypad::new();

        assert!(joypad.directions == 0xFF);
        assert!(joypad.actions == 0xFF);

        joypad.update(&controller);

        assert!(joypad.directions == 0);
        assert!(joypad.actions == 0);
    }
}
