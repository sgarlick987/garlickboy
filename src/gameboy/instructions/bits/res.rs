use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// RES 0,B - 0x80
// Length: 2 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (8t)
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
        let bit = !(1 << bit);
        let byte = gameboy.registers.get_from_enum(&target);
        gameboy.registers.set_from_enum(&target, byte & bit);
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_already_reset() {
        for target in TARGETS {
            for bit in 0..8 {
                let gameboy = &mut Gameboy::new();
                let byte = !(1u8 << bit);
                gameboy.registers.set_from_enum(&target, byte);
                let steps = new(&bit, &target);
                assert_eq!(steps.len(), CYCLES);

                for step in steps {
                    gameboy.execute(step);
                }

                assert_eq!(gameboy.pc, LENGTH);
                assert_eq!(
                    gameboy.registers.get_from_enum(&target),
                    byte,
                    "bit should be 0 if already reset"
                );
            }
        }
    }

    #[test]
    fn test_reset() {
        for target in TARGETS {
            for bit in 0..8 {
                let gameboy = &mut Gameboy::new();
                let byte = 0xFF;
                let check = !(1u8 << bit);
                gameboy.registers.set_from_enum(&target, byte);
                let steps = new(&bit, &target);
                assert_eq!(steps.len(), CYCLES);

                for step in steps {
                    gameboy.execute(step);
                }

                assert_eq!(gameboy.pc, LENGTH);
                assert_eq!(
                    gameboy.registers.get_from_enum(&target),
                    check,
                    "bit should be 0 if reset"
                );
            }
        }
    }
}
