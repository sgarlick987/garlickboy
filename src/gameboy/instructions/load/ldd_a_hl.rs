use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// LD A,(HL-) - 0x3A
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
// read	(HL--)->A
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        gameboy.registers.a = gameboy.read_byte(hl);
        gameboy.registers.set_hl(hl.wrapping_sub(1));
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
