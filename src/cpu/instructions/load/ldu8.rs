// LD B,u8 - 0x06
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (8t)
// fetch
// read	u8->B
//     pub fn ld_u8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let value = self.read_byte_pc_lower();
//         match target {
//             TargetRegister8::A => self.registers.a = value,
//             TargetRegister8::B => self.registers.b = value,
//             TargetRegister8::C => self.registers.c = value,
//             TargetRegister8::D => self.registers.d = value,
//             TargetRegister8::E => self.registers.e = value,
//             TargetRegister8::H => self.registers.h = value,
//             TargetRegister8::L => self.registers.l = value,
//         }

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

use std::collections::VecDeque;

use crate::cpu::{instructions::TargetRegister8, GameboyChip};

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

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let byte = chip.read_byte_pc_lower();
            chip.set_register_from_enum(&inst.target, byte);
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
