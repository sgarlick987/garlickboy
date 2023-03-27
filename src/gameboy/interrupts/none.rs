use crate::gameboy::GameboyCycle;

struct Interrupt {}

impl Iterator for Interrupt {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        return None;
    }
}

impl ExactSizeIterator for Interrupt {
    fn len(&self) -> usize {
        0
    }
}

pub fn new() -> Box<dyn ExactSizeIterator<Item = GameboyCycle>> {
    Box::new(Interrupt {})
}
