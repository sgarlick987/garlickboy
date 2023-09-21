use crate::{
    gameboy::Gameboy,
    gameboy::{instructions::TargetRegister16, GameboyCycle, GameboyCycles},
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// LD BC,u16 - 0x01
// Length: 3 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timingwithout branch (12t)
// fetch
// read	u16:lower->C
// read	u16:upper->B
struct Context {
    lower: u8,
    upper: u8,
    target: TargetRegister16,
}

pub fn new(target: &TargetRegister16) -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        upper: 0,
        lower: 0,
        target: target.clone(),
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

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

        match context.target {
            TargetRegister16::SP => {
                gameboy.registers.p = context.lower;
                gameboy.registers.s = context.upper;
            }
            TargetRegister16::HL => {
                gameboy.registers.l = context.lower;
                gameboy.registers.h = context.upper;
            }
            TargetRegister16::DE => {
                gameboy.registers.e = context.lower;
                gameboy.registers.d = context.upper;
            }
            TargetRegister16::BC => {
                gameboy.registers.c = context.lower;
                gameboy.registers.b = context.upper;
            }
        }

        gameboy.pc = gameboy.pc.wrapping_add(3);
    }));

    Box::new(cycles.into_iter())
}
