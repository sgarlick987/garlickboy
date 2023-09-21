use crate::{
    gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles},
    utils::sub_bytes_half_carry,
};
use std::collections::VecDeque;

// CP A,B - 0xB8
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);
    let target = target.clone();

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.registers.get_from_enum(&target);
        let a = gameboy.registers.a;
        gameboy.set_negative_flag();
        gameboy.write_zero_flag(a == byte);
        gameboy.write_carry_flag(a < byte);
        gameboy.write_half_carry_flag(sub_bytes_half_carry(a, byte));
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
