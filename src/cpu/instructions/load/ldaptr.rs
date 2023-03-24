use std::collections::VecDeque;

use crate::cpu::{instructions::TargetPointer, GameboyChip};

// LD A,(BC) - 0x0A
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (8t)
// fetch
// read	(BC)->A
struct Inst {
    target: TargetPointer,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(target: &TargetPointer) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            chip.registers.a = match inst.target {
                TargetPointer::BC => chip.read_byte(chip.registers.get_bc()),
                TargetPointer::DE => chip.read_byte(chip.registers.get_de()),
                TargetPointer::HL => chip.read_byte(chip.registers.get_hl()),
            };
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
