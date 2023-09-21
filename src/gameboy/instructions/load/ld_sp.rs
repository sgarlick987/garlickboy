use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::{merge_bytes, split_bytes},
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// LD (u16),SP - 0x08
// Length: 3 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timing
// without branch (20t)
// fetch
// read	u16:lower
// read	u16:upper
// write	SP:lower->(u16)
// write	SP:upper->(u16+1)
struct Context {
    lower: u8,
    address: u16,
    sp_upper: u8,
}

pub fn new() -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        address: 0,
        sp_upper: 0,
        lower: 0,
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(5);

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
        let upper = gameboy.read_byte_pc_upper();
        context.address = merge_bytes(upper, context.lower);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        let (upper, lower) = split_bytes(gameboy.registers.get_sp());
        context.sp_upper = upper;
        gameboy.write_byte(context.address, lower);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.write_byte(context.address.wrapping_add(1), context.sp_upper);
        gameboy.pc = gameboy.pc.wrapping_add(3);
    }));

    Box::new(cycles.into_iter())
}
