use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// CCF - 0x3F
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(|gameboy: &mut Gameboy| {
        let carry = !gameboy.carry_flag();
        gameboy.reset_negative_flag();
        gameboy.reset_half_carry_flag();
        gameboy.write_carry_flag(carry);
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
    fn test_set_carry() {
        let gameboy = &mut Gameboy::new();
        gameboy.reset_carry_flag();
        let steps = new();
        assert_eq!(steps.len(), CYCLES);

        for step in steps {
            gameboy.execute(step);
        }

        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.carry_flag(), true);
    }

    #[test]
    fn test_reset_carry() {
        let gameboy = &mut Gameboy::new();
        gameboy.set_carry_flag();
        let steps = new();
        assert_eq!(steps.len(), CYCLES);

        for step in steps {
            gameboy.execute(step);
        }

        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.carry_flag(), false);
    }
}
