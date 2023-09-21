use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// JP HL - 0xE9
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// with branch (4t)
// fetch
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.pc = gameboy.registers.get_hl();
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use coverage_helper::test;

    const CYCLES: usize = 1;
    const JP_ADDRESS: u16 = 0xFF00;

    #[test]
    fn test_jp_hl() {
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        gameboy.registers.set_hl(JP_ADDRESS);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, JP_ADDRESS);
    }
}
