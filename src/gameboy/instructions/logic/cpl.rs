use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

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
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.registers.a ^= 0xFF;
        gameboy.set_negative_flag();
        gameboy.set_half_carry_flag();
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
