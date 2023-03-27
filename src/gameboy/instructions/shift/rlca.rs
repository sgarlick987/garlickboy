use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// RLCA - 0x07
// Length: 1 byte
// Flags
// Zero	unset
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/rsb
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
            gameboy.registers.a = gameboy.registers.a.rotate_left(1);

            gameboy.update_carry_flag(gameboy.registers.a >> 7 == 1);
            gameboy.reset_half_carry_flag();
            gameboy.reset_negative_flag();
            gameboy.reset_zero_flag();

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
