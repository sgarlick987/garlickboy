use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

use super::sub;

// SBC A,(HL) - 0x9E
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (8t)
// fetch
// read	(HL)
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        let byte = gameboy.read_byte(hl);
        let carry = gameboy.carry_flag();
        gameboy.registers.a = sub(gameboy, byte, carry);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
