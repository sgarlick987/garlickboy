use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

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
            let register = chip.registers.get_from_enum(&inst.target);
            let mut value = register << 1;
            if chip.registers.flags.carry {
                value |= 1;
            }
            chip.registers.set_from_enum(&inst.target, value);

            chip.registers.flags.carry = register >> 7 == 1;
            chip.registers.flags.zero = value == 0;
            chip.registers.flags.half_carry = false;
            chip.registers.flags.negative = false;

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
