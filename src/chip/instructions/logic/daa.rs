// DAA - 0x27
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unmodified
// Half Carry	unset
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
use std::collections::VecDeque;

use crate::chip::GameboyChip;

struct Inst {
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let mut register = chip.registers.a;
            if chip.negative_flag() {
                if chip.carry_flag() {
                    register -= 0x60;
                }
                if chip.half_carry_flag() {
                    register -= 0x6;
                }
            } else {
                if chip.carry_flag() || register > 0x99 {
                    register += 0x60;
                    chip.set_carry_flag();
                }
                if chip.half_carry_flag() || (register & 0xF) > 0x9 {
                    register += 0x6;
                }
            }

            chip.registers.a = register;
            chip.update_zero_flag(register == 0);
            chip.reset_half_carry_flag();
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
