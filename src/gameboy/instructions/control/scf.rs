use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// SCF - 0x37
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unset
// Half Carry	unset
// Carry	set
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(|gameboy: &mut Gameboy| {
        gameboy.reset_negative_flag();
        gameboy.reset_half_carry_flag();
        gameboy.set_carry_flag();
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

    fn test(gameboy: &mut Gameboy, steps: GameboyCycles) {
        assert_eq!(steps.len(), CYCLES);

        for inst in steps {
            gameboy.execute(inst);
        }

        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.negative_flag(), false);
        assert_eq!(gameboy.carry_flag(), true);
        assert_eq!(gameboy.half_carry_flag(), false);
    }

    #[test]
    fn test_sets() {
        let gameboy = &mut Gameboy::new();
        gameboy.set_negative_flag();
        gameboy.reset_carry_flag();
        gameboy.set_negative_flag();
        let steps = new();
        test(gameboy, steps);
    }

    #[test]
    fn test_already_set() {
        let gameboy = &mut Gameboy::new();
        gameboy.reset_negative_flag();
        gameboy.set_carry_flag();
        gameboy.reset_half_carry_flag();
        let steps = new();
        test(gameboy, steps);
    }
}
