use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle};

// RES 0,B - 0x80
// Length: 2 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (8t)
// fetch	(0xCB)
// fetch
struct Inst {
    bit: u8,
    target: TargetRegister8,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(bit: &u8, target: &TargetRegister8) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        bit: *bit,
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let bit = !1 << inst.bit;
            let mut register = gameboy.registers.get_from_enum(&inst.target);
            register &= bit;
            gameboy.registers.set_from_enum(&inst.target, register);
            gameboy.pc = gameboy.pc.wrapping_add(2);
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
