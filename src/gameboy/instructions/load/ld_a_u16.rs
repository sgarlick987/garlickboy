use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::merge_bytes,
};

// LD A,(u16) - 0xFA
// Length: 3 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timing
// without branch (16t)
// fetch
// read	u16:lower
// read	u16:upper
// read	(u16)->A.
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

    let inst_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.lower = gameboy.read_byte_pc_lower();
    }));

    let inst_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.upper = gameboy.read_byte_pc_upper();
    }));

    let inst_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow();
        let address = merge_bytes(inst.upper, inst.lower);
        gameboy.registers.a = gameboy.read_byte(address);
        gameboy.pc = gameboy.pc.wrapping_add(3);
    }));

    Box::new(cycles.into_iter())
}
