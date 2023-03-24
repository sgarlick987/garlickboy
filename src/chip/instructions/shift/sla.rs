use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

// SLA B - 0x20
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	dependent
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

    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {}));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let mut byte = chip.registers.get_from_enum(&inst.target);
            chip.registers.flags.carry = byte >> 7 == 1;
            byte = byte << 1;
            chip.registers.set_from_enum(&inst.target, byte);

            chip.registers.flags.zero = byte == 0;
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

//     pub fn sla(&mut self, target: &TargetRegister8) {
//         //fetch

//         //fetch

//         self.pc = self.pc.wrapping_add(2);
//     }
// }
