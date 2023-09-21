use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// LD A,(FF00+u8) - 0xF0
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (12t)
// fetch
// read	u8
// read	(FF00+u8)->A
pub fn new() -> GameboyCycles {
    let address = Rc::new(RefCell::new(0u16));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let address_ref = address.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        address_ref.replace(0xFF00 + gameboy.read_byte_pc_lower() as u16);
    }));

    let address_ref = address.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.registers.a = gameboy.read_byte(address_ref.take());
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
