use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

// AND A,B - 0xA0
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unset
// Group: x8/alu
// Timing
// without branch (4t)
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
            let byte = chip.registers.get_from_enum(&inst.target);
            chip.registers.a &= byte;
            chip.reset_negative_flag();
            chip.reset_carry_flag();
            chip.update_zero_flag(chip.registers.a == 0);
            chip.set_half_carry_flag();
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
