use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// LD E,(HL) - 0x5E
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
// read	(HL)->E
pub fn new(target: &TargetRegister8) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);
    let target = target.clone();

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = gameboy.read_byte(gameboy.registers.get_hl());
        gameboy.registers.set_from_enum(&target, byte);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
