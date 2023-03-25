use std::collections::VecDeque;

use crate::{chip::GameboyChip, utils::sub_bytes_half_carry};

// CP A,(HL) - 0xBE
// Length: 1 byte
// FlagsZero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timingwithout branch (8t)
// fetch
// read	(HL)
struct Inst {
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(2),
    };

    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {}));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let hl = chip.registers.get_hl();
            let byte = chip.read_byte(hl);
            let a = chip.registers.a;
            chip.set_negative_flag();
            chip.update_zero_flag(a == byte);
            chip.update_carry_flag(a < byte);
            chip.update_half_carry_flag(sub_bytes_half_carry(a, byte));

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
