use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};

// AND A,B - 0xA0
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unset
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);
    let target = target.clone();

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.registers.get_from_enum(&target);
        gameboy.registers.a &= byte;
        gameboy.reset_negative_flag();
        gameboy.reset_carry_flag();
        gameboy.write_zero_flag(gameboy.registers.a == 0);
        gameboy.set_half_carry_flag();
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
