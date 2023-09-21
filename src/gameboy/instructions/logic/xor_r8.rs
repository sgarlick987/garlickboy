use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// XOR A,B - 0xA8
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);
    let target = target.clone();

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.registers.a ^= gameboy.registers.get_from_enum(&target);
        gameboy.write_zero_flag(gameboy.registers.a == 0);
        gameboy.reset_negative_flag();
        gameboy.reset_half_carry_flag();
        gameboy.reset_carry_flag();
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
