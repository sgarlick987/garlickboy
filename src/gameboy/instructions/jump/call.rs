use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::{merge_bytes, split_bytes},
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// CALL u16 - 0xCD
// Length: 3 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timingwith branch (24t)
// fetch
// read	u16:lower
// read	u16:upper
// internal	branch decision?
// write	PC:upper->(--SP)
// write	PC:lower->(--SP)
struct Context {
    upper: u8,
    lower: u8,
    return_upper: u8,
    return_lower: u8,
}

pub fn new() -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        upper: 0,
        lower: 0,
        return_upper: 0,
        return_lower: 0,
    }));

    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(6);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
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
        let mut context = context_ref.borrow_mut();
        (context.return_upper, context.return_lower) = split_bytes(gameboy.pc.wrapping_add(3));
        let address = merge_bytes(context.upper, context.lower);
        gameboy.pc = address;
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.push(context.return_upper);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.push(context.return_lower);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::{bios::BIOS_MAPPED_ADDRESS, bus::HRAM_ADDRESS_START};
    use coverage_helper::test;

    const CYCLES: usize = 6;
    const CALL_ADDRESS: u16 = 0xFF00;
    const STACK_ADDRESS: u16 = 0xFFFE;
    const PC: u16 = HRAM_ADDRESS_START;

    #[test]
    fn test_call() {
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        let (upper, lower) = split_bytes(CALL_ADDRESS);
        gameboy.registers.set_sp(STACK_ADDRESS);
        gameboy.pc = PC;
        gameboy.write_byte(BIOS_MAPPED_ADDRESS, 1);
        gameboy.write_byte(gameboy.pc + 1, lower);
        gameboy.write_byte(gameboy.pc + 2, upper);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, CALL_ADDRESS);
        let (upper, lower) = split_bytes(PC + 3);
        assert_eq!(gameboy.pop(), lower);
        assert_eq!(gameboy.pop(), upper);
    }
}
