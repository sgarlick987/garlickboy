use std::collections::VecDeque;

use crate::{
    gameboy::{Gameboy, GameboyCycle},
    utils::sub_bytes_half_carry,
};

// CP A,(HL) - 0xBE
// Length: 1 byte
// FlagsZero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timingwithout branch (8t)
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
            let a = gameboy.registers.a;
            gameboy.set_negative_flag();
            gameboy.write_zero_flag(a == byte);
            gameboy.write_carry_flag(a < byte);
            gameboy.write_half_carry_flag(sub_bytes_half_carry(a, byte));

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
