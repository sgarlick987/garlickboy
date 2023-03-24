//     // CP A,u8 - 0xFE
//     // Length: 2 bytes
//     // FlagsZero	dependent
//     // Negative	set
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timingwithout branch (8t)
//     // fetch
//     // read	u8
//     pub fn cp_u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read

//         cycles_used += self.sync();
//         cycles_used
//     }

use std::collections::VecDeque;

use crate::{cpu::GameboyChip, utils::bytes_half_carry};

struct Inst {
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {}));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let byte = chip.read_byte_pc_lower();
            let a = chip.registers.a;
            chip.registers.flags.negative = true;
            chip.registers.flags.zero = a == byte;
            chip.registers.flags.carry = a < byte;
            chip.registers.flags.half_carry = bytes_half_carry(a, byte);

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
