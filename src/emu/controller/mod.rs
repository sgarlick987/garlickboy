mod keyboard;

use keyboard::Keyboard;
use sdl2::EventPump;

const DOWN_BIT: u8 = 1 << 3;
const UP_BIT: u8 = 1 << 2;
const LEFT_BIT: u8 = 1 << 1;
const RIGHT_BIT: u8 = 1;
const START_BIT: u8 = 1 << 3;
const SELECT_BIT: u8 = 1 << 2;
const B_BIT: u8 = 1 << 1;
const A_BIT: u8 = 1;

pub trait Controller {
    fn actions(&self) -> u8;
    fn directions(&self) -> u8;
    fn read(&mut self, event_pump: &EventPump);
}

pub fn new_keyboard_controller() -> Box<dyn Controller> {
    Keyboard::new()
}
