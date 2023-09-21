use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// SWAP (HL) - 0x36
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	unset
// Group: x8/rsb
// Timing
// without branch (16t)
// fetch	(0xCB)
// fetch
// read	(HL)
// write	(HL)
pub fn new() -> GameboyCycles {
    let byte = Rc::new(RefCell::new(0u8));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        let byte = gameboy.read_byte(hl);
        byte_ref.replace(byte);
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = byte_ref.take();
        let swapped = (byte << 4) | (byte >> 4);
        gameboy.write_byte(gameboy.registers.get_hl(), swapped);
        gameboy.write_zero_flag(swapped == 0);
        gameboy.reset_negative_flag();
        gameboy.reset_half_carry_flag();
        gameboy.reset_carry_flag();
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
