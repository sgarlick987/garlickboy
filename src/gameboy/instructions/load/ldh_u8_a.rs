use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// LD (FF00+u8),A - 0xE0
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (12t)
// fetch
// read	u8
// write	A->(FF00+u8)
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
        gameboy.write_byte(address_ref.take(), gameboy.registers.a);
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
