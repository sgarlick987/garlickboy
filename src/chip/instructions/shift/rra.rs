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
            let carry_in = chip.carry_flag() as u8;
            let carry_out = register & 1 == 1;
            let byte = (register >> 1) | (carry_in << 7);
            chip.registers.a = byte;

            chip.update_carry_flag(carry_out);
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
