use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// NOP - 0x00
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/misc
// Timingwithout branch (4t)
// fetch
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(|gameboy: &mut Gameboy| {
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use coverage_helper::test;

    const LENGTH: u16 = 1;
    const CYCLES: usize = 1;

    #[test]
    fn test_nop() {
        let mut gameboy = Gameboy::new();
        let steps = new();
        assert_eq!(steps.len(), CYCLES);

        for inst in steps {
            gameboy.execute(inst);
        }

        assert_eq!(gameboy.pc, LENGTH);
    }
}
