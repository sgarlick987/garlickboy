use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

// XOR A,B - 0xA8
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
struct Inst {
    target: TargetRegister8,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    target: &TargetRegister8,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            chip.registers.a ^= chip.registers.get_from_enum(&inst.target);
            chip.update_zero_flag(chip.registers.a == 0);
            chip.reset_negative_flag();
            chip.reset_half_carry_flag();
            chip.reset_carry_flag();
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
