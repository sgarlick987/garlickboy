use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

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
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
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

    Box::new(cycles.into_iter())
}
