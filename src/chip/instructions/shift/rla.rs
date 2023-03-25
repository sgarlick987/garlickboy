use std::collections::VecDeque;

use crate::chip::GameboyChip;

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
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let register = chip.registers.a;
            let mut value = register << 1;
            if chip.carry_flag() {
                value |= 1;
            }
            chip.registers.a = value;

            chip.update_carry_flag(register >> 7 == 1);
            chip.reset_zero_flag();
            chip.reset_half_carry_flag();
            chip.reset_negative_flag();

            chip.pc = chip.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

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
    use crate::chip::{bus::MockBus, GameboyChip};

    fn setup_chip() -> GameboyChip {
        let bus = Box::new(MockBus::new());
        GameboyChip::new(bus)
    }

    #[test]
    fn test_rla_carryin_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut chip = setup_chip();
        chip.registers.a = 0b10000011; //carryout
        chip.set_carry_flag(); //carryin

        let mut cycles = 0;
        for step in new() {
            chip.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(chip.pc, LENGTH);
        assert_eq!(chip.registers.a, 0b00000111);
        assert!(chip.carry_flag(), "carry flag should be set");
        assert!(!chip.zero_flag(), "zero flag should not be set");
        assert!(!chip.half_carry_flag(), "half carry flag should not be set");
        assert!(!chip.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rla_no_carryin_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut chip = setup_chip();
        chip.registers.a = 0b10000011; //carryout
        chip.reset_carry_flag(); //no carryin

        let mut cycles = 0;
        for step in new() {
            chip.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(chip.pc, LENGTH);
        assert_eq!(chip.registers.a, 0b00000110);
        assert!(chip.carry_flag(), "carry flag should be set");
        assert!(!chip.zero_flag(), "zero flag should not be set");
        assert!(!chip.half_carry_flag(), "half carry flag should not be set");
        assert!(!chip.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rla_carryin_no_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut chip = setup_chip();
        chip.registers.a = 0b00000011; //no carryout
        chip.set_carry_flag(); //carryin

        let mut cycles = 0;
        for step in new() {
            chip.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(chip.pc, LENGTH);
        assert_eq!(chip.registers.a, 0b00000111);
        assert!(!chip.carry_flag(), "carry flag should not be set");
        assert!(!chip.zero_flag(), "zero flag should not be set");
        assert!(!chip.half_carry_flag(), "half carry flag should not be set");
        assert!(!chip.negative_flag(), "negative flag should not be set");
    }

    #[test]
    fn test_rla_no_carryin_no_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 1;

        let mut chip = setup_chip();
        chip.registers.a = 0b00000011; //no carryout
        chip.reset_carry_flag(); //no carryin

        let mut cycles = 0;
        for step in new() {
            chip.execute(step);
            cycles += 1;
        }

        assert_eq!(cycles, CYCLES);
        assert_eq!(chip.pc, LENGTH);
        assert_eq!(chip.registers.a, 0b00000110);
        assert!(!chip.carry_flag(), "carry flag should not be set");
        assert!(!chip.zero_flag(), "zero flag should not be set");
        assert!(!chip.half_carry_flag(), "half carry flag should not be set");
        assert!(!chip.negative_flag(), "negative flag should not be set");
    }
}
