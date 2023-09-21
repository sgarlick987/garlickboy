use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

use super::add;

// ADC A,u8 - 0xCE
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (8t)
// fetch
// read	u8
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.read_byte_pc_lower();
        let carry = gameboy.carry_flag();
        gameboy.registers.a = add(gameboy, byte, carry);
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
