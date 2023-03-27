use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// RRA - 0x1F
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
            let register = gameboy.registers.a;
            let carry_in = gameboy.carry_flag() as u8;
            let carry_out = register & 1 == 1;
            let byte = (register >> 1) | (carry_in << 7);
            gameboy.registers.a = byte;

            gameboy.update_carry_flag(carry_out);
            gameboy.reset_zero_flag();
            gameboy.reset_half_carry_flag();
            gameboy.reset_negative_flag();

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
