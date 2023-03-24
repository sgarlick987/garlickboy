use std::collections::VecDeque;

use crate::cpu::GameboyChip;

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
            if chip.registers.flags.carry {
                value |= 1;
            }
            chip.registers.a = value;

            chip.registers.flags.carry = register >> 7 == 1;
            chip.registers.flags.zero = false;
            chip.registers.flags.half_carry = false;
            chip.registers.flags.negative = false;

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
