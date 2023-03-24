use std::collections::VecDeque;

use crate::cpu::{instructions::TargetRegister8, GameboyChip};

// XOR A,B - 0xA8
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
// Group: x8/alu
// Timingwithout branch (4t)
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
            chip.registers.a ^= chip.registers.get_from_enum(&inst.target);
            chip.registers.flags.zero = chip.registers.a == 0;
            chip.registers.flags.negative = false;
            chip.registers.flags.half_carry = false;
            chip.registers.flags.carry = false;
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

//     // XOR A,u8 - 0xEE
//     // Length: 2 bytes
//     // Flags
//     // Zero	dependent
//     // Negative	unset
//     // Half Carry	unset
//     // Carry	unset
//     // Group: x8/alu
//     // Timing
//     // without branch (8t)
//     // fetch
//     // read	u8
//     pub fn xor_u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte_pc_lower();

//         self.registers.a ^= byte;
//         self.registers.flags.zero = self.registers.a == 0;
//         self.registers.flags.negative = false;
//         self.registers.flags.half_carry = false;
//         self.registers.flags.carry = false;

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }
