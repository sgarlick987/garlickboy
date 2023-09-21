use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::add_bytes_half_carry,
};

// LD HL,SP+i8 - 0xF8
// Length: 2 bytes
// Flags
// Zero	unset
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x16/alu
// Timing
// without branch (12t)
// fetch
// read	i8
// internal
pub fn new() -> GameboyCycles {
    let byte = Rc::new(RefCell::new(0i8));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        byte_ref.replace(gameboy.read_byte_pc_lower() as i8);
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = byte_ref.take();
        let sp = gameboy.registers.get_sp();
        let value = sp.wrapping_add(byte as u16);
        gameboy.registers.set_hl(value);
        gameboy.reset_negative_flag();
        gameboy.reset_zero_flag();
        gameboy.write_carry_flag((sp & 0x00FF) + (byte as u16 & 0x00FF) > 0x00FF);
        gameboy.write_half_carry_flag(add_bytes_half_carry(gameboy.registers.p, byte as u8));
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
