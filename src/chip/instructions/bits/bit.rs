use std::collections::VecDeque;

use crate::chip::{instructions::TargetRegister8, GameboyChip};

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
            chip.registers.flags.zero = chip.registers.get_from_enum(&inst.target) & check == 0;
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
