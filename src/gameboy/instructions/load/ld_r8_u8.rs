use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};

// LD D,u8 - 0x16
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (8t)
// fetch
// read	u8->D
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);
    let target = target.clone();

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.read_byte_pc_lower();
        gameboy.registers.set_from_enum(&target, byte);
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
