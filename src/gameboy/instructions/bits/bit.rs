use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// BIT 2,B - 0x50
// Length: 2 bytes
// FlagsZero	dependent
// Negative	unset
// Half Carry	set
// Carry	unmodified
// Group: x8/rsb
// Timingwithout branch (8t)
// fetch	(0xCB)
// fetch
pub fn new(bit: &u8, target: &TargetRegister8) -> GameboyCycles {
    let bit = bit.clone();
    let target = target.clone();
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.registers.get_from_enum(&target);
        super::bit(gameboy, byte, bit);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::instructions::bits::tests::{test_bit_not_set, test_bit_set};
    use coverage_helper::test;

    const TARGETS: [TargetRegister8; 7] = [
        TargetRegister8::A,
        TargetRegister8::B,
        TargetRegister8::C,
        TargetRegister8::D,
        TargetRegister8::E,
        TargetRegister8::H,
        TargetRegister8::L,
    ];

    const LENGTH: u16 = 2;
    const CYCLES: usize = 2;

    #[test]
    fn test_not_set() {
        for target in TARGETS {
            for bit in 0..8 {
                let gameboy = &mut Gameboy::new();
                let check = 0xFF ^ (1 << bit);
                gameboy.registers.set_from_enum(&target, check);
                let steps = new(&bit, &target);
                test_bit_not_set(gameboy, steps, CYCLES, LENGTH);
            }
        }
    }

    #[test]
    fn test_set() {
        for target in TARGETS {
            for bit in 0..8 {
                let gameboy = &mut Gameboy::new();
                let check = 1 << bit;
                gameboy.registers.set_from_enum(&target, check);
                let cycles = new(&bit, &target);
                test_bit_set(gameboy, cycles, CYCLES, LENGTH);
            }
        }
    }
}
