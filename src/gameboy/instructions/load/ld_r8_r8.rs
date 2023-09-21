use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// LD B,B - 0x40
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (4t)
// fetch
pub fn new(target: &TargetRegister8, source: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);
    let target = target.clone();
    let source = source.clone();

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let value = gameboy.registers.get_from_enum(&source);
        gameboy.registers.set_from_enum(&target, value);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
