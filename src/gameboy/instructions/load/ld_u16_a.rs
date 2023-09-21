use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::merge_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// LD (u16),A - 0xEA
// Length: 3 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (16t)
// fetch
// read	u16:lower
// read	u16:upper
// write	A->(u16)
struct Context {
    lower: u8,
    upper: u8,
}

pub fn new() -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context { upper: 0, lower: 0 }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.lower = gameboy.read_byte_pc_lower();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.upper = gameboy.read_byte_pc_upper();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        let address = merge_bytes(context.upper, context.lower);
        gameboy.write_byte(address, gameboy.registers.a);
        gameboy.pc = gameboy.pc.wrapping_add(3);
    }));

    Box::new(cycles.into_iter())
}
