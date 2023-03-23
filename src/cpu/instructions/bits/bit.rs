use std::collections::VecDeque;

use crate::cpu::{instructions::TargetRegister8, GameboyChip};

//     // BIT 2,B - 0x50
//     // Length: 2 bytes
//     // FlagsZero	dependent
//     // Negative	unset
//     // Half Carry	set
//     // Carry	unmodified
//     // Group: x8/rsb
//     // Timingwithout branch (8t)
//     // fetch	(0xCB)
//     // fetch
//     pub fn bit(&mut self, bit: &u8, target: &TargetRegister8) {
//         //fetch

//         let check = 1 << bit;

//         //fetch
//         match target {
//             TargetRegister8::A => self.registers.flags.zero = self.registers.a & check == 0,
//             TargetRegister8::B => self.registers.flags.zero = self.registers.b & check == 0,
//             TargetRegister8::C => self.registers.flags.zero = self.registers.c & check == 0,
//             TargetRegister8::D => self.registers.flags.zero = self.registers.d & check == 0,
//             TargetRegister8::E => self.registers.flags.zero = self.registers.e & check == 0,
//             TargetRegister8::H => self.registers.flags.zero = self.registers.h & check == 0,
//             TargetRegister8::L => self.registers.flags.zero = self.registers.l & check == 0,
//         }
//         self.registers.flags.negative = false;
//         self.registers.flags.half_carry = true;

//         self.pc = self.pc.wrapping_add(2);
//     }

struct Inst {
    bit: u8,
    target: TargetRegister8,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    bit: &u8,
    target: &TargetRegister8,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        bit: *bit,
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let check = 1 << inst.bit;
            chip.registers.flags.zero = chip.get_register_from_enum(&inst.target) & check == 0;
            chip.registers.flags.negative = false;
            chip.registers.flags.half_carry = true;

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
