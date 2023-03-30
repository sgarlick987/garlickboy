use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// XOR A,(HL) - 0xAE
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
// Group: x8/alu
// Timing
// without branch (8t)
// fetch
// read	(HL)
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
            let hl = gameboy.registers.get_hl();
            let byte = gameboy.read_byte(hl);
            let a = gameboy.registers.a ^ byte;

            gameboy.registers.a = a;
            gameboy.write_zero_flag(a == 0);
            gameboy.reset_negative_flag();
            gameboy.reset_carry_flag();
            gameboy.reset_half_carry_flag();
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
