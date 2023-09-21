use crate::{
    gameboy::{instructions::TargetRegister16, Gameboy, GameboyCycle, GameboyCycles},
    utils::split_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// ADD HL,DE - 0x19
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x16/alu
// Timing
// without branch (8t)
// fetch	Probably writes to L here
// internal	Probably writes to H here
struct Context {
    upper: u8,
    target: TargetRegister16,
}

pub fn new(target: &TargetRegister16) -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        upper: 0,
        target: target.clone(),
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        let hl = gameboy.registers.get_hl();
        let value = match context.target {
            TargetRegister16::DE => gameboy.registers.get_de(),
            TargetRegister16::HL => gameboy.registers.get_hl(),
            TargetRegister16::BC => gameboy.registers.get_bc(),
            TargetRegister16::SP => gameboy.registers.get_sp(),
        };
        let (added, overflowed) = hl.overflowing_add(value);
        let (upper, lower) = split_bytes(added);
        gameboy.registers.l = lower;
        gameboy.write_carry_flag(overflowed);
        gameboy.write_half_carry_flag((hl & 0xFFF) + (value & 0xFFF) > 0xFFF);
        context.upper = upper;
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.registers.h = context.upper;
        gameboy.reset_negative_flag();
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}
