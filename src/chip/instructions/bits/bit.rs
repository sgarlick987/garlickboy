use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

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
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    bit: &u8,
    target: &TargetRegister8,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        bit: *bit,
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let check = 1 << inst.bit;
            let register = chip.registers.get_from_enum(&inst.target);

            chip.update_zero_flag(register & check == 0);
            chip.reset_negative_flag();
            chip.set_half_carry_flag();
            //carry unmodified

            chip.pc = chip.pc.wrapping_add(2);
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
    use crate::chip::bus::MockBus;
    use coverage_helper::test;

    use super::*;

    fn setup_chip() -> GameboyChip {
        let bus = Box::new(MockBus::new());
        GameboyChip::new(bus)
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
                let mut chip = setup_chip();
                let check = 0xFF ^ (1 << bit);

                chip.registers.set_from_enum(&target, check);
                for step in new(&bit, &target) {
                    chip.execute(step);
                }

                assert_eq!(chip.pc, LENGTH);
                assert!(chip.zero_flag(), "zero flag should be set");
                assert!(chip.half_carry_flag(), "half carry flag should be set");
                assert!(!chip.negative_flag(), "negative flag should not be set");
            }
        }
    }

    #[test]
    fn test_bit_set() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 2;

        for target in TARGETS {
            for bit in 0..8 {
                let mut chip = setup_chip();
                let check = 1 << bit;

                chip.registers.set_from_enum(&target, check);
                for step in new(&bit, &target) {
                    chip.execute(step);
                }

                assert_eq!(chip.pc, LENGTH);
                assert!(!chip.zero_flag(), "zero flag should not be set");
                assert!(chip.half_carry_flag(), "half carry flag should be set");
                assert!(!chip.negative_flag(), "negative flag should not be set");
            }
        }
    }
}
