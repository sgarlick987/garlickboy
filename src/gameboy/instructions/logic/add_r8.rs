use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

use super::add;

// ADD A,B - 0x80
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);
    let target = target.clone();

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let register = gameboy.registers.get_from_enum(&target);
        gameboy.registers.a = add(gameboy, register, false);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
