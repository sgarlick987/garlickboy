use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// AND A,u8 - 0xE6
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unset
// Group: x8/alu
// Timing
// without branch (8t)
// fetch
// read	u8
struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(2),
    };

    inst.executions
        .push_back(Box::new(move |_: &mut Gameboy| {}));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let byte = gameboy.read_byte_pc_lower();
            gameboy.registers.a &= byte;

            gameboy.reset_negative_flag();
            gameboy.reset_carry_flag();
            gameboy.write_zero_flag(gameboy.registers.a == 0);
            gameboy.set_half_carry_flag();
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
