use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};

// SLA (HL) - 0x26
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	dependent
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
        let carry_out = byte & 1 == 1;
        let msb = byte >> 7 << 7;
        let value = (byte >> 1) | msb;
        gameboy.write_byte(gameboy.registers.get_hl(), value);
        gameboy.write_carry_flag(carry_out);
        gameboy.write_zero_flag(value == 0);
        gameboy.reset_half_carry_flag();
        gameboy.reset_negative_flag();
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
