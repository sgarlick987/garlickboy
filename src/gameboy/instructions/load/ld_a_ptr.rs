use crate::gameboy::{instructions::TargetPointer, Gameboy, GameboyCycle, GameboyCycles};
use std::collections::VecDeque;

// LD A,(BC) - 0x0A
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (8t)
// fetch
// read	(BC)->A
pub fn new(target: &TargetPointer) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);
    let target = target.clone();

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.registers.a = match target {
            TargetPointer::BC => gameboy.read_byte(gameboy.registers.get_bc()),
            TargetPointer::DE => gameboy.read_byte(gameboy.registers.get_de()),
        };
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
