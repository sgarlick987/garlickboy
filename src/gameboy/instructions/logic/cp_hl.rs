use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::sub_bytes_half_carry,
};
use std::collections::VecDeque;

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
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {}));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        let byte = gameboy.read_byte(hl);
        let a = gameboy.registers.a;
        gameboy.set_negative_flag();
        gameboy.write_zero_flag(a == byte);
        gameboy.write_carry_flag(a < byte);
        gameboy.write_half_carry_flag(sub_bytes_half_carry(a, byte));

        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
