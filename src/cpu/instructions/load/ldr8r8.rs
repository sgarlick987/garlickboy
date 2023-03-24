use std::collections::VecDeque;

use crate::cpu::{instructions::TargetRegister8, GameboyChip};

// LD B,B - 0x40
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (4t)
// fetch
struct Inst {
    target: TargetRegister8,
    source: TargetRegister8,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    target: &TargetRegister8,
    source: &TargetRegister8,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        source: source.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let value = chip.registers.get_from_enum(&inst.source);
            chip.registers.set_from_enum(&inst.target, value);
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
