use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// AND A,(HL) - 0xA6
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unset
// Group: x8/alu
// Timing
// without branch (8t)
// fetch
// read	(HL)
pub fn new() -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {}));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        let byte = gameboy.read_byte(hl);
        gameboy.registers.a &= byte;

        gameboy.reset_negative_flag();
        gameboy.reset_carry_flag();
        gameboy.write_zero_flag(gameboy.registers.a == 0);
        gameboy.set_half_carry_flag();
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
