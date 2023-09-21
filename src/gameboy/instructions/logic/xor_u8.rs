use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// XOR A,u8 - 0xEE
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
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
        gameboy.registers.a ^= gameboy.read_byte_pc_lower();
        gameboy.write_zero_flag(gameboy.registers.a == 0);
        gameboy.reset_negative_flag();
        gameboy.reset_half_carry_flag();
        gameboy.reset_carry_flag();
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
