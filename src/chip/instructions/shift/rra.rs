use std::collections::VecDeque;

use crate::chip::GameboyChip;

// RRA - 0x1F
// Length: 1 byte
// Flags
// Zero	unset
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/rsb
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
            let register = chip.registers.a;
            let mut value = register >> 1;
            if chip.carry_flag() {
                value |= 1 << 7;
            }
            chip.registers.a = value;

            chip.update_carry_flag(register & 1 == 1);
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
