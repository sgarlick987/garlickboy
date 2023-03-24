use std::collections::VecDeque;

use crate::{cpu::GameboyChip, utils::bytes_half_carry};

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
            chip.registers.flags.negative = true;
            chip.registers.flags.zero = a == byte;
            chip.registers.flags.carry = a < byte;
            chip.registers.flags.half_carry = bytes_half_carry(a, byte);

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

//     pub fn cp_hl(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let hl = self.registers.get_hl();
//         let byte = self.read_byte(hl);
//         let a = self.registers.a;
//         self.registers.flags.negative = true;
//         self.registers.flags.zero = a == byte;
//         self.registers.flags.carry = a < byte;
//         self.registers.flags.half_carry = bytes_half_carry(a, byte);

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }
