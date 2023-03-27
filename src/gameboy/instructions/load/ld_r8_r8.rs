use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle};

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
    executions: VecDeque<GameboyCycle>,
}

pub fn new(
    target: &TargetRegister8,
    source: &TargetRegister8,
) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        source: source.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let value = gameboy.registers.get_from_enum(&inst.source);
            gameboy.registers.set_from_enum(&inst.target, value);
            gameboy.pc = gameboy.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
