use std::collections::VecDeque;

use crate::{
    chip::{instructions::TargetRegister8, GameboyChip},
    utils::sub_bytes_half_carry,
};

// CP A,B - 0xB8
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
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

//     pub fn cp_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }
