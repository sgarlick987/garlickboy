use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle};

// RL C - 0x11
// Length: 2 bytes
// FlagsZero	dependent
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/rsb
// Timingwithout branch (8t)
// fetch	(0xCB)
// fetch
struct Inst {
    target: TargetRegister8,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetRegister8) -> Box<dyn ExactSizeIterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let register = gameboy.registers.get_from_enum(&inst.target);
            let carry_in = gameboy.carry_flag() as u8;
            let carry_out = register >> 7 == 1;
            let byte = (register << 1) | carry_in;
            gameboy.registers.set_from_enum(&inst.target, byte);

            gameboy.update_carry_flag(carry_out);
            gameboy.update_zero_flag(byte == 0);
            gameboy.reset_half_carry_flag();
            gameboy.reset_negative_flag();

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

impl ExactSizeIterator for Inst {
    fn len(&self) -> usize {
        self.executions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::{instructions::TargetRegister8, Gameboy};

    fn setup_gameboy() -> Gameboy {
        Gameboy::new()
    }

    #[test]
    fn test_rl_carryin_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: usize = 2;

        let mut cpu = setup_gameboy();
        cpu.registers.c = 0b10000011; //carryout
        cpu.set_carry_flag(); //carryin

        let rl = new(&TargetRegister8::C);
        assert_eq!(rl.len(), CYCLES);
        for step in rl {
            cpu.execute(step);
        }

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000111);
        assert!(cpu.carry_flag(), "carry flag should be set");
        assert!(!cpu.zero_flag(), "zero flag should not be set");
        assert!(!cpu.half_carry_flag(), "half carry flag should not be set");
        assert!(!cpu.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rl_carryin_no_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: usize = 2;

        let mut cpu = setup_gameboy();
        cpu.registers.c = 0b00000011; // no carryout
        cpu.set_carry_flag(); //carryin

        let rl = new(&TargetRegister8::C);
        assert_eq!(rl.len(), CYCLES);
        for step in rl {
            cpu.execute(step);
        }

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000111);
        assert!(!cpu.carry_flag(), "carry flag should not be set");
        assert!(!cpu.zero_flag(), "zero flag should not be set");
        assert!(!cpu.half_carry_flag(), "half carry flag should not be set");
        assert!(!cpu.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rl_no_carryin_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: usize = 2;

        let mut cpu = setup_gameboy();
        cpu.registers.c = 0b10000011; //carryout
        cpu.reset_carry_flag(); //no carryin

        let rl = new(&TargetRegister8::C);
        assert_eq!(rl.len(), CYCLES);
        for step in rl {
            cpu.execute(step);
        }

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000110);
        assert!(cpu.carry_flag(), "carry flag should be set");
        assert!(!cpu.zero_flag(), "zero flag should not be set");
        assert!(!cpu.half_carry_flag(), "half carry flag should not be set");
        assert!(!cpu.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rl_no_carryin_no_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: usize = 2;

        let mut cpu = setup_gameboy();
        cpu.registers.c = 0b00000011; //no carryout
        cpu.reset_carry_flag(); //no carryin

        let rl = new(&TargetRegister8::C);
        assert_eq!(rl.len(), CYCLES);
        for step in rl {
            cpu.execute(step);
        }

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000110);
        assert!(!cpu.carry_flag(), "carry flag should not be set");
        assert!(!cpu.zero_flag(), "zero flag should not be set");
        assert!(!cpu.half_carry_flag(), "half carry flag should not be set");
        assert!(!cpu.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rl_zero() {
        const LENGTH: u16 = 2;
        const CYCLES: usize = 2;

        let mut cpu = setup_gameboy();
        cpu.registers.c = 0b00000000; //no carryout
        cpu.reset_carry_flag(); //no carryin

        let rl = new(&TargetRegister8::C);
        assert_eq!(rl.len(), CYCLES);
        for step in rl {
            cpu.execute(step);
        }

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000000);
        assert!(!cpu.carry_flag(), "carry flag should not be set");
        assert!(cpu.zero_flag(), "zero flag should be set");
        assert!(!cpu.half_carry_flag(), "half carry flag should not be set");
        assert!(!cpu.negative_flag(), "negative flag should not be set");
    }
}
