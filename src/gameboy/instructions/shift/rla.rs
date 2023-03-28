use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// RLA - 0x17
// Length: 1 byte
// FlagsZero	unset
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/rsb
// Timingwithout branch (4t)
// fetch
struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let register = gameboy.registers.a;
            let carry_in = gameboy.carry_flag() as u8;
            let carry_out = register >> 7 == 1;
            let byte = (register << 1) | carry_in;
            gameboy.registers.a = byte;

            gameboy.update_carry_flag(carry_out);
            gameboy.reset_zero_flag();
            gameboy.reset_half_carry_flag();
            gameboy.reset_negative_flag();

            gameboy.pc = gameboy.pc.wrapping_add(1);
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
    use super::*;
    use crate::gameboy::Gameboy;

    fn setup_gameboy() -> Gameboy {
        Gameboy::new()
    }

    #[test]
    fn test_rla_carryin_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut gameboy = setup_gameboy();
        gameboy.registers.a = 0b10000011; //carryout
        gameboy.set_carry_flag(); //carryin

        let mut cycles = 0;
        for step in new() {
            gameboy.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.registers.a, 0b00000111);
        assert!(gameboy.carry_flag(), "carry flag should be set");
        assert!(!gameboy.zero_flag(), "zero flag should not be set");
        assert!(
            !gameboy.half_carry_flag(),
            "half carry flag should not be set"
        );
        assert!(!gameboy.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rla_no_carryin_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut gameboy = setup_gameboy();
        gameboy.registers.a = 0b10000011; //carryout
        gameboy.reset_carry_flag(); //no carryin

        let mut cycles = 0;
        for step in new() {
            gameboy.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.registers.a, 0b00000110);
        assert!(gameboy.carry_flag(), "carry flag should be set");
        assert!(!gameboy.zero_flag(), "zero flag should not be set");
        assert!(
            !gameboy.half_carry_flag(),
            "half carry flag should not be set"
        );
        assert!(!gameboy.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rla_carryin_no_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut gameboy = setup_gameboy();
        gameboy.registers.a = 0b00000011; //no carryout
        gameboy.set_carry_flag(); //carryin

        let mut cycles = 0;
        for step in new() {
            gameboy.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.registers.a, 0b00000111);
        assert!(!gameboy.carry_flag(), "carry flag should not be set");
        assert!(!gameboy.zero_flag(), "zero flag should not be set");
        assert!(
            !gameboy.half_carry_flag(),
            "half carry flag should not be set"
        );
        assert!(!gameboy.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rla_no_carryin_no_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut gameboy = setup_gameboy();
        gameboy.registers.a = 0b00000011; //no carryout
        gameboy.reset_carry_flag(); //no carryin

        let mut cycles = 0;
        for step in new() {
            gameboy.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(gameboy.pc, LENGTH);
        assert_eq!(gameboy.registers.a, 0b00000110);
        assert!(!gameboy.carry_flag(), "carry flag should not be set");
        assert!(!gameboy.zero_flag(), "zero flag should not be set");
        assert!(
            !gameboy.half_carry_flag(),
            "half carry flag should not be set"
        );
        assert!(!gameboy.negative_flag(), "negative flag should not be set");
    }
}
