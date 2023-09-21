use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

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
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let register = gameboy.registers.a;
        let value = register.rotate_left(1);
        gameboy.registers.a = value;
        gameboy.write_carry_flag(register >> 7 == 1);
        gameboy.reset_half_carry_flag();
        gameboy.reset_negative_flag();
        gameboy.reset_zero_flag();
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
