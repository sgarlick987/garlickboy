use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

use super::sub;

// SBC A,B - 0x98
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
        let register = gameboy.registers.get_from_enum(&target);
        let carry = gameboy.carry_flag();
        gameboy.registers.a = sub(gameboy, register, carry);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
