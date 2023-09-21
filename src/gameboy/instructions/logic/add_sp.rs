use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::{add_bytes_half_carry, split_bytes},
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// ADD SP,i8 - 0xE8
// Length: 2 bytes
// Flags
// Zero	unset
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x16/alu
// Timing
// without branch (16t)
// fetch
// read	i8
// internal	Probably writes to SP:lower here
// write	Probably writes to SP:upper here
struct Context {
    lower: u8,
    upper: u8,
}

pub fn new() -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context { lower: 0, upper: 0 }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        let sp = gameboy.registers.get_sp();
        let byte = gameboy.read_byte_pc_lower() as i8;

        let added = sp.wrapping_add(byte as u16);
        (context.upper, context.lower) = split_bytes(added);

        gameboy.reset_negative_flag();
        gameboy.reset_zero_flag();
        gameboy.write_carry_flag((sp & 0x00FF) + (byte as u16 & 0x00FF) > 0x00FF);
        gameboy.write_half_carry_flag(add_bytes_half_carry(gameboy.registers.p, byte as u8));
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.registers.p = context.lower;
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.registers.s = context.upper;
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
