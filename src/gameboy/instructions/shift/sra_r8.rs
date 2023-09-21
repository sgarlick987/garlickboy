use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// SRA C - 0x29
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/rsb
// Timing
// without branch (8t)
// fetch	(0xCB)
// fetch
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);
    let target = target.clone();

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let register = gameboy.registers.get_from_enum(&target);
        let carry_out = register & 1 == 1;
        let msb = register >> 7 << 7;
        let byte = (register >> 1) | msb;
        gameboy.registers.set_from_enum(&target, byte);
        gameboy.write_carry_flag(carry_out);
        gameboy.write_zero_flag(byte == 0);
        gameboy.reset_half_carry_flag();
        gameboy.reset_negative_flag();
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
