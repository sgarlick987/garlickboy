use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

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
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let register = gameboy.registers.a;
        let carry_in = gameboy.carry_flag() as u8;
        let carry_out = register & 1 == 1;
        let byte = (register >> 1) | (carry_in << 7);
        gameboy.registers.a = byte;

        gameboy.write_carry_flag(carry_out);
        gameboy.reset_zero_flag();
        gameboy.reset_half_carry_flag();
        gameboy.reset_negative_flag();

        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
