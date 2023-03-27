use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// CPL - 0x2F
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	set
// Half Carry	set
// Carry	unmodified
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            gameboy.registers.a ^= 0xFF;
            gameboy.set_negative_flag();
            gameboy.set_half_carry_flag();

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
