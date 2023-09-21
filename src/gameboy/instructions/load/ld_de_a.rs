use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// LD (DE),A - 0x12
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timing
// without branch (8t)
// fetch
// write	A->(DE)
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.write_byte(gameboy.registers.get_de(), gameboy.registers.a);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
