use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// LD (HL+),A - 0x22
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (8t)
// fetch
// write	A->(HL++)
struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let hl = gameboy.registers.get_hl();
            gameboy.write_byte(hl, gameboy.registers.a);
            gameboy.registers.set_hl(hl.wrapping_add(1));
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