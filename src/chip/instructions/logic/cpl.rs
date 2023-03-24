use std::collections::VecDeque;

use crate::chip::GameboyChip;

// CPL - 0x2F
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	set
// Half Carry	set
// Carry	unmodified
// Group: x8/alu
// Timing
// without branch (4t)
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
            chip.registers.a ^= 0xFF;
            chip.registers.flags.negative = true;
            chip.registers.flags.half_carry = true;

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
