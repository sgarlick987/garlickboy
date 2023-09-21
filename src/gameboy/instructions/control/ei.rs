use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// EI - 0xFB
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/misc
// Timing
// without branch (4t)
// fetch
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(|gameboy: &mut Gameboy| {
        gameboy.schedule_ime();
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
    fn test_ei() {
        let mut gameboy = Gameboy::new();
        let cycles = new();
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, LENGTH);
    }
}
