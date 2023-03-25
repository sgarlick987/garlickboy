use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

// SWAP B - 0x30
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
// Group: x8/rsb
// Timing
// without branch (8t)
// fetch	(0xCB)
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
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let swapped = chip.registers.get_from_enum(&inst.target).swap_bytes();
            chip.registers.set_from_enum(&inst.target, swapped);
            chip.registers.flags.zero = swapped == 0;
            chip.registers.flags.negative = false;
            chip.registers.flags.half_carry = false;
            chip.registers.flags.carry = false;

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
