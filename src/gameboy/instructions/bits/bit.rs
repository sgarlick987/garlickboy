use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle};

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
struct Inst {
    bit: u8,
    target: TargetRegister8,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(bit: &u8, target: &TargetRegister8) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        bit: *bit,
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let check = 1 << inst.bit;
            let register = gameboy.registers.get_from_enum(&inst.target);

            gameboy.update_zero_flag(register & check == 0);
            gameboy.reset_negative_flag();
            gameboy.set_half_carry_flag();
            //carry unmodified

            gameboy.pc = gameboy.pc.wrapping_add(2);
        }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}

#[cfg(test)]
mod tests {

    use coverage_helper::test;

    use super::*;

    fn setup_gameboy() -> Gameboy {
        Gameboy::new()
    }
    const TARGETS: [TargetRegister8; 7] = [
        TargetRegister8::A,
        TargetRegister8::B,
        TargetRegister8::C,
        TargetRegister8::D,
        TargetRegister8::E,
        TargetRegister8::H,
        TargetRegister8::L,
    ];

    #[test]
    fn test_bit_not_set() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 2;

        for target in TARGETS {
            for bit in 0..8 {
                let mut gameboy = setup_gameboy();
                let check = 0xFF ^ (1 << bit);

                gameboy.registers.set_from_enum(&target, check);
                for step in new(&bit, &target) {
                    gameboy.execute(step);
                }

                assert_eq!(gameboy.pc, LENGTH);
                assert!(gameboy.zero_flag(), "zero flag should be set");
                assert!(gameboy.half_carry_flag(), "half carry flag should be set");
                assert!(!gameboy.negative_flag(), "negative flag should not be set");
            }
        }
    }

    #[test]
    fn test_bit_set() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 2;

        for target in TARGETS {
            for bit in 0..8 {
                let mut gameboy = setup_gameboy();
                let check = 1 << bit;

                gameboy.registers.set_from_enum(&target, check);
                for step in new(&bit, &target) {
                    gameboy.execute(step);
                }

                assert_eq!(gameboy.pc, LENGTH);
                assert!(!gameboy.zero_flag(), "zero flag should not be set");
                assert!(gameboy.half_carry_flag(), "half carry flag should be set");
                assert!(!gameboy.negative_flag(), "negative flag should not be set");
            }
        }
    }
}
