use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::sub_bytes_half_carry,
};
use std::collections::VecDeque;

// CP A,u8 - 0xFE
// Length: 2 bytes
// FlagsZero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timingwithout branch (8t)
// fetch
// read	u8
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.read_byte_pc_lower();
        let a = gameboy.registers.a;
        gameboy.set_negative_flag();
        gameboy.write_zero_flag(a == byte);
        gameboy.write_carry_flag(a < byte);
        gameboy.write_half_carry_flag(sub_bytes_half_carry(a, byte));
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
