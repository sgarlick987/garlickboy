use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// LD (DE),A - 0x12
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timing
// without branch (8t)
// fetch
// write	A->(DE)
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
            gameboy.write_byte(gameboy.registers.get_de(), gameboy.registers.a);
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