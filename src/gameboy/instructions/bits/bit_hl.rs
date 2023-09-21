use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// BIT 0,(HL) - 0x46
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (12t)
// fetch	(0xCB)
// fetch
// read	(HL)
pub fn new(bit: &u8) -> GameboyCycles {
    let bit = bit.clone();
    let mut cycle: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    cycle.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycle.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycle.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        let byte = gameboy.read_byte(hl);
        super::bit(gameboy, byte, bit);
    }));

    Box::new(cycle.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::{
        bus::HRAM_ADDRESS_START,
        instructions::bits::tests::{test_bit_not_set, test_bit_set},
    };
    use coverage_helper::test;

    const LENGTH: u16 = 2;
    const CYCLES: usize = 3;

    #[test]
    fn test_not_set() {
        for bit in 0..8 {
            let gameboy = &mut Gameboy::new();
            let check = 0xFF ^ (1 << bit);
            gameboy.registers.set_hl(HRAM_ADDRESS_START);
            gameboy.write_byte(gameboy.registers.get_hl(), check);
            let cycles = new(&bit);
            test_bit_not_set(gameboy, cycles, CYCLES, LENGTH);
        }
    }

    #[test]
    fn test_set() {
        for bit in 0..8 {
            let gameboy = &mut Gameboy::new();
            let check = 1 << bit;
            gameboy.registers.set_hl(HRAM_ADDRESS_START);
            gameboy.write_byte(gameboy.registers.get_hl(), check);
            let steps = new(&bit);
            test_bit_set(gameboy, steps, CYCLES, LENGTH);
        }
    }
}
